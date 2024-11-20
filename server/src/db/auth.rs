use sqlx::query_scalar;
use uuid::Uuid;

use crate::AppState;
use rand::Rng;

pub async fn create_refresh_token(
    user_id: Uuid,
    ctx: &AppState
) -> Result<std::string::String, sqlx::Error> {
    let token: String = rand
        ::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(64)
        .map(char::from)
        .collect();
    let expires_at = (chrono::Utc::now() + chrono::Duration::days(30)).naive_utc();

    match
        query_scalar!(
            r#"insert into refresh_tokens (token, user_id, expires_at) values ($1, $2, $3) returning token"#,
            token,
            user_id,
            expires_at
        ).fetch_one(&ctx.db).await
    {
        Ok(token) => Ok(token),
        Err(e) => Err(e),
    }
}
