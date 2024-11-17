use sqlx::query;
use uuid::Uuid;

use crate::{ models::user::User, state::AppState };

pub async fn get_user_by_email(email: &str, ctx: &AppState) -> Result<User, sqlx::Error> {
    query!(r#"SELECT user_id, email, password FROM users WHERE email = $1"#, &email)
        .fetch_one(&ctx.db).await
        .map(|record| User {
            user_id: record.user_id,
            email: record.email,
            password: record.password,
        })
}

pub async fn get_user_by_id(id: &Uuid, ctx: &AppState) -> Result<User, sqlx::Error> {
    query!(r#"SELECT user_id, email, password FROM users WHERE user_id = $1"#, &id)
        .fetch_one(&ctx.db).await
        .map(|record| User {
            user_id: record.user_id,
            email: record.email,
            password: record.password,
        })
}
