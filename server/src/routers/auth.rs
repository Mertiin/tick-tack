use axum::http::StatusCode;
use axum::routing::post;
use axum::{ middleware, Extension, Json };
use axum::{ response::IntoResponse, Router };
use log::debug;
use pwhash::bcrypt;

use crate::auth::access_token::encode_jwt;
use crate::auth::authorization_middleware::auth;
use crate::db::auth::create_refresh_token;
use crate::db::user::get_user_by_email;
use crate::AppState;
use sqlx::query;

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
        debug!("User logged in: {}", _e);
        bcrypt::verify("req.password", "&user.password");
        let error_response =
            serde_json::json!({
            "status": "error",
            "message": "Invalid email or password",
        });
        (StatusCode::UNAUTHORIZED, Json(error_response))
    })?;

    debug!("User logged in: {}", user.email);

    if bcrypt::verify(req.password, &user.password) {
        let refresh_token = match create_refresh_token(user.id, &ctx).await {
            Ok(token) => token,
            Err(e) => {
                let error_response =
                    serde_json::json!({
                "status": "error",
                "message": format!("Failed to create refresh token: {}", e),
            });
                debug!("User logged in: {}", error_response);

                return Ok((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
            }
        };

        debug!("User logged in: {}", refresh_token);

        encode_jwt(user.id.to_string(), user.email)
            .map(|access_token| {
                debug!("User logged in: {}", access_token);
                let response =
                    serde_json::json!({
            "status": "success",
            "accessToken": access_token,
            "refreshToken": refresh_token,
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

#[allow(non_snake_case)]
#[derive(serde::Deserialize)]
struct GetAccessTokenRequest {
    refreshToken: String,
}

#[axum::debug_handler]
async fn get_access_token(
    ctx: Extension<AppState>,
    Json(req): Json<GetAccessTokenRequest>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user = match
        query!(
            r#"SELECT users.id, email FROM refresh_tokens
        JOIN users ON refresh_tokens.user_id = users.id
         WHERE token = $1 AND expires_at > NOW()"#,
            &req.refreshToken
        ).fetch_one(&ctx.db).await
    {
        Ok(user) => user,
        Err(_e) => {
            let error_response =
                serde_json::json!({
                    "status": "error",
                    "message": "Failed to token",
                });
            return Ok((StatusCode::UNAUTHORIZED, Json(error_response)));
        }
    };

    match encode_jwt(user.id.to_string(), user.email) {
        Ok(access_token) => {
            let response =
                serde_json::json!({
                    "status": "success",
                    "accessToken": access_token,
                });
            Ok((StatusCode::OK, Json(response)))
        }
        Err(e) => {
            let error_response =
                serde_json::json!({
                    "status": "error",
                    "message": format!("JWT error: {}", e),
                });
            Ok((StatusCode::UNAUTHORIZED, Json(error_response)))
        }
    }
}

#[derive(serde::Deserialize)]
struct LogoutRequest {
    refresh_token: String,
}

#[axum::debug_handler]
async fn logout(
    ctx: Extension<AppState>,
    Json(req): Json<LogoutRequest>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    query!("DELETE FROM refresh_tokens WHERE token = $1", &req.refresh_token)
        .execute(&ctx.db).await
        .map_err(|_e| {
            let error_response =
                serde_json::json!({
            "status": "error",
            "message": "Failed to delete refresh token",
        });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let response =
        serde_json::json!({
        "status": "success",
        "message": "Logged out",
    });
    Ok((StatusCode::OK, Json(response)))
}

pub fn router() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/access_token", post(get_access_token))
        .route("/logout", post(logout).layer(middleware::from_fn(auth)))
}
