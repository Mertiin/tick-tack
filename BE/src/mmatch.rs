use redis::Commands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct MatchData {
    pub player_one: String,
    pub player_two: Option<String>,
    pub turn: PlayerMark,
    pub board: Vec<Vec<PlayerMark>>,
}



#[derive(Serialize, Deserialize, PartialEq, Clone)] 
pub enum PlayerMark {
    Cross,
    Circle,
    Empty,
}

impl PlayerMark {
    pub fn as_str(&self) -> &str {
        match self {
            PlayerMark::Cross => "cross",
            PlayerMark::Circle => "circle",
            PlayerMark::Empty => "",
        }
    }
}

pub fn get_redis_connection() -> redis::Connection {
    let client = redis::Client::open("redis://redis:6379/").expect("Failed to create Redis client");
    let con = client.get_connection().expect("Failed to connect to Redis");
    con
}

pub fn create_match(match_id: &String, player_one: Uuid) -> Result<MatchData, String> {
    let mut con = get_redis_connection();

    let mut board: Vec<Vec<PlayerMark>> = vec![];
    for _ in 0..9 {
        let mut row: Vec<PlayerMark> = vec![];
        for _ in 0..9 {
            row.push(PlayerMark::Empty);
        }
        board.push(row);
    }

    let data = MatchData {
        player_one: player_one.to_string(),
        player_two: None,
        turn: PlayerMark::Cross,
        board,
    };
    let match_data_json = serde_json::json!(data);

    let _: () = con.set_ex(match_id.to_string(), match_data_json.to_string(), 7200).map_err(|e| e.to_string())?;

    Ok(data)
}

pub fn get_match(match_id: &String) -> Result<MatchData, String> {
    let mut con = get_redis_connection();

    let return_json: String = con.get(match_id).map_err(|e| e.to_string())?;
    let match_data: MatchData = serde_json::from_str(&return_json).map_err(|e| e.to_string())?;

    Ok(match_data)
}

pub fn update_match(match_id: &String, player_two: Uuid) -> Result<MatchData, String> {
    let mut con = get_redis_connection();

    let return_json: String = con.get(match_id).map_err(|e| e.to_string())?;
    let mut match_data: MatchData = serde_json::from_str(&return_json).map_err(|e| e.to_string())?;
    match_data.player_two = Some(player_two.to_string());

    let value = serde_json::to_string(&match_data).map_err(|e| e.to_string())?;
    let _: () = con.set(match_id, value).map_err(|e| e.to_string())?;

    Ok(match_data)
}

pub fn make_move(match_id: &String, mark: PlayerMark, x: usize, y: usize) -> Result<MatchData, String> {
    let mut con = get_redis_connection();

    let return_json: String = con.get(match_id).map_err(|e| e.to_string())?;
    let mut match_data: MatchData = serde_json::from_str(&return_json).map_err(|e| e.to_string())?;

    if match_data.board[x][y] != PlayerMark::Empty {
        return Err("Invalid move".to_string());
    }

    match_data.board[x][y] = mark.clone();
    match_data.turn = match mark.clone() {
        PlayerMark::Cross => PlayerMark::Circle,
        PlayerMark::Circle => PlayerMark::Cross,
        PlayerMark::Empty => PlayerMark::Empty,
    };

    let value = serde_json::to_string(&match_data).map_err(|e| e.to_string())?;
    let _: () = con.set(match_id, value).map_err(|e| e.to_string())?;

    Ok(match_data)
}