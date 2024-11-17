use axum::http::StatusCode;
use axum::routing::post;
use axum::{ Extension, Json };
use axum::{ response::IntoResponse, Router };
use pwhash::bcrypt;
use sqlx::query_scalar;

use crate::auth::access_token::encode_jwt;
use crate::db::user::get_user_by_email;
use crate::AppState;
use rand::Rng;

#[derive(serde::Deserialize)]
struct LoginUser {
    email: String,
    password: String,
}

#[axum::debug_handler]
async fn login(
    ctx: Extension<AppState>,
    Json(req): Json<LoginUser>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = get_user_by_email(&req.email, &ctx).await.map_err(|_e| {
        bcrypt::verify("req.password", "&user.password");
        let error_response =
            serde_json::json!({
            "status": "error",
            "message": "Invalid email or password",
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    if bcrypt::verify(req.password, &user.password) {
        let token: String = rand
            ::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();

        let expires_at = (chrono::Utc::now() + chrono::Duration::days(30)).naive_utc();

        let refresh_token = query_scalar!(
            r#"insert into refresh_tokens (token, user_id, expires_at) values ($1, $2, $3) returning token"#,
            token,
            user.user_id,
            expires_at
        )
            .fetch_one(&ctx.db).await
            .map_err(|_e| {
                let error_response =
                    serde_json::json!({
            "status": "error",
            "message": "Failed to create user",
        });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })?;

        encode_jwt(user.user_id.to_string(), user.email)
            .map(|token| {
                let response =
                    serde_json::json!({
            "status": "success",
            "token": token,
            "refresh_token": refresh_token,
        });
                (StatusCode::OK, Json(response))
            })
            .map_err(|e| {
                let error_response =
                    serde_json::json!({
        "status": "error",
        "message": format!("JWT error: {}", e),
    });
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
            })
    } else {
        let error_response =
            serde_json::json!({
        "status": "error",
        "message": "Invalid email or password",
    });
        Ok((StatusCode::UNAUTHORIZED, Json(error_response)))
    }
}

pub fn router() -> Router {
    Router::new().route("/", post(login))
}
