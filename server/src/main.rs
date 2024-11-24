mod models;
mod routers;
mod state;
mod auth;
mod db;

use axum::{ response::IntoResponse, routing::get, Router, Extension };
use fastwebsockets::upgrade;
use fastwebsockets::OpCode;
use fastwebsockets::WebSocketError;
use log::{ error, info };
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;
use state::AppState;
use tower_http::cors::CorsLayer;

use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "debug");
    // Initialize the logger
    env_logger::init();

    let pool = match
        PgPoolOptions::new()
            .max_connections(5)
            .connect(env::var("DATABASE_URL").unwrap().as_str()).await
    {
        Ok(pool) => {
            info!("Connected to the database");
            pool
        }
        Err(e) => {
            error!("Failed to connect to the database: {}", e);
            unimplemented!()
        }
    };

    let shared_state = AppState { db: pool };
    let app = Router::new()
        .route("/ws", get(ws_handler))
        .nest("/api", routers::router())
        .layer(CorsLayer::permissive())
        .layer(Extension(shared_state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app.into_make_service()).await.unwrap();
}

async fn handle_client(fut: upgrade::UpgradeFut) -> Result<(), WebSocketError> {
    let mut ws = fastwebsockets::FragmentCollector::new(fut.await?);
    loop {
        let frame = ws.read_frame().await?;
        match frame.opcode {
            OpCode::Close => {
                break;
            }
            OpCode::Text | OpCode::Binary => {
                info!("Received message: {:?}", frame.payload);
                ws.write_frame(frame).await?;
            }
            _ => {}
        }
    }

    Ok(())
}

async fn ws_handler(ws: upgrade::IncomingUpgrade) -> impl IntoResponse {
    let (response, fut) = ws.upgrade().unwrap();

    tokio::task::spawn(async move {
        if let Err(e) = handle_client(fut).await {
            eprintln!("Error in websocket connection: {}", e);
        }
    });

    response
}
