use uuid::Uuid;
use sqlx::FromRow;

#[derive(FromRow, serde::Serialize, serde::Deserialize, Clone)]
pub struct User {
    pub user_id: Uuid,
    pub email: String,
    pub password: String,
}
