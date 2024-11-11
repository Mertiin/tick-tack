mod mmatch;

use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tokio_tungstenite::tungstenite::http::Uri;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
use tokio_tungstenite::accept_hdr_async;
use futures::{SinkExt, StreamExt};
use uuid::Uuid;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use std::collections::HashMap;
use log::{error, info};
use crate::mmatch::{create_match, get_match, make_move, MatchData, PlayerMark}; 

pub struct Client {
    pub user_id: String,
    pub match_id: String,
    pub sender: futures::stream::SplitSink<tokio_tungstenite::WebSocketStream<TcpStream>, Message>,
}

type Clients = Arc<Mutex<HashMap<String, HashMap<String ,Client>>>>;

#[tokio::main]
async fn main() {
    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    std::env::set_var("RUST_LOG", "debug");
    // Initialize the logger
    env_logger::init();

    // Get the address to bind to    
    let addr: SocketAddr = "0.0.0.0:8080".parse().expect("Invalid address");

    // Create the TCP listener
    let listener = TcpListener::bind(&addr).await.expect("Failed to bind");

    info!("Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        info!("Accepted connection");
        // Spawn a new task for each connection
        tokio::spawn(handle_connection(stream, clients.clone()));
    }
}
async fn handle_connection(stream: TcpStream, matches: Clients) {
    let user_id = Uuid::new_v4();   

    let mut path: Uri = "/".parse().unwrap();
    // Accept the WebSocket connection
    let ws_stream = match accept_hdr_async(stream, |req: &Request, resp: Response| {
        path = req.uri().clone();
        Ok(resp)
    }).await {
        Ok(result) => result,
        Err(e) => {
            error!("Error during the websocket handshake: {}", e);
            return;
        }
    };

    // Split the WebSocket stream into a sender and receiver
    let (mut sender, mut receiver) = ws_stream.split();

    let initial_message = serde_json::json!({
        "type": "user_id",
        "userId": user_id.to_string(),
    });

    sender.send(Message::Text(initial_message.to_string())).await.ok();

    // let sender_clone = Arc::clone(&sender);
    let match_id: String;
    let re = regex::Regex::new(r"^/match/([a-z-]+)$").unwrap();
    if let Some(captures) = re.captures(path.path()) {
        if let Some(group_one) = captures.get(1) {
            match_id = group_one.as_str().to_string();
            let mut game = get_match(&match_id);
            if game.is_ok() {
                info!("Game found");
            }
            else if game.is_err() {
                info!("Game not found, creating new game");

                game = create_match(&match_id, user_id);
            }


            let data: MatchData = game.unwrap();
            let player_mark = if data.player_one == user_id.to_string() {
                PlayerMark::Cross
            } else {
                PlayerMark::Circle
            };

            let message = serde_json::json!({
                "type": "initial_match_data",
               "board": &data.board,
                "turn": data.turn,
                "mark": player_mark,
            });

            sender.send(Message::Text(message.to_string())).await.ok();
        }else  {
            error!("Invalid match ID");
            return sender.close().await.unwrap();
        }
    }else {
        error!("Invalid path");
        return sender.close().await.unwrap();
    }

    let client = Client {
        user_id: user_id.to_string(),
        match_id: match_id.clone(),
        sender: sender,
    };
    
    let mut locked_match = matches.lock().await;
    
    if let Some(mmatch) = locked_match.get_mut(&match_id.to_string()) {
     mmatch.insert(user_id.to_string(), client);
    } else {
        let mut mmatch = HashMap::new();
        mmatch.insert(user_id.to_string(), client);

        locked_match.insert( match_id.to_string(), mmatch);
    }
    
    drop(locked_match);

    // Handle incoming messages
    while let Some(msg) = receiver.next().await {
        match msg {
            Ok(Message::Text(text)) => {    
                // Reverse the received string and send it back
                if let Some(clients) = matches.lock().await.get_mut(&match_id) {
                    let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
                    if parsed["type"] == "move" {
                        let grid = parsed["grid"].as_i64().unwrap();
                        let x = parsed["x"].as_i64().unwrap();
                        let y = parsed["y"].as_i64().unwrap();
                        let index = y * 3 + x;

                        info!("Received move: grid {}, index {}", grid, index);

                        let mmatch = get_match(&match_id).unwrap();

                        let mark = if mmatch.turn == PlayerMark::Cross {
                            PlayerMark::Cross
                        } else {
                            PlayerMark::Circle
                        };

                        match make_move(&match_id, mark, grid.try_into().unwrap(), index.try_into().unwrap()) {
                            Ok(board) => {
                                let message = serde_json::json!({
                                    "type": "move",
                                    "board": board.board,
                                    "turn": board.turn,
                                    "activeGrid": {
                                        "x": x,
                                        "y": y,
                                    }
                                });

                                for (client_id, client) in clients.iter_mut() {
                                    client.sender.send(Message::Text(message.to_string())).await.ok();
                                }
                            }
                            Err(e) => {
                                error!("Error making move: {}", e);
                            }
                        }
                    }
                }  
            }
            Ok(Message::Close(_)) => break,
            Ok(_) => (),
            Err(e) => {
                error!("Error processing message: {}", e);
                break;
            }
        }
    }
}