use sqlx::{ query, query_scalar };
use uuid::Uuid;

use crate::{ models::user::User, state::AppState };

use super::auth::create_refresh_token;

pub async fn get_user_by_email(email: &str, ctx: &AppState) -> Result<User, sqlx::Error> {
    query!(r#"SELECT id, email, password FROM users WHERE email = $1"#, &email)
        .fetch_one(&ctx.db).await
        .map(|record| User {
            id: record.id,
            email: record.email,
            password: record.password,
        })
}

pub async fn get_user_by_id(id: &Uuid, ctx: &AppState) -> Result<User, sqlx::Error> {
    query!(r#"SELECT id, email, password FROM users WHERE id = $1"#, &id)
        .fetch_one(&ctx.db).await
        .map(|record| User {
            id: record.id,
            email: record.email,
            password: record.password,
        })
}

pub async fn create_user(
    email: &str,
    hashed_password: &str,
    ctx: &AppState
) -> Result<Uuid, sqlx::Error> {
    match
        query_scalar!(
            // language=PostgreSQL
            r#"insert into users (email, password) values ($1, $2) returning id"#,
            email,
            hashed_password
        ).fetch_one(&ctx.db).await
    {
        Ok(id) => Ok(id),
        Err(e) => {
            return Err(e);
        }
    }
}
