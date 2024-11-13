use axum::http::StatusCode;
use axum::{ Extension, Json };
use axum::{ response::IntoResponse, routing::get, Router };
use uuid::Uuid;

use crate::{ AppState };

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct UserModel {
    pub user_id: Uuid,
    pub name: String,
}

#[axum::debug_handler]
async fn get_users(
    ctx: Extension<AppState>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let users = sqlx
        ::query_as::<_, UserModel>(r#"SELECT * FROM users"#)
        .fetch_all(&ctx.db).await
        .map_err(|e| {
            let error_response =
                serde_json::json!({
            "status": "error",
            "message": format!("Database error: { }", e),
        });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let json_response =
        serde_json::json!({
        "status": "ok",
        "count": users.len(),
        "users": users
    });

    Ok(Json(json_response))
}

#[derive(serde::Deserialize)]
struct NewUser {
    name: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct UserBody<T> {
    user: T,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct User {
    user_id: uuid::Uuid,
    name: String,
}

async fn post_users(
    ctx: Extension<AppState>,
    Json(req): Json<UserBody<NewUser>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = sqlx
        ::query_scalar(
            // language=PostgreSQL
            r#"insert into users (name) values ($1) returning user_id"#
        )
        .bind(req.user.name.clone())
        .fetch_one(&ctx.db).await
        .map_err(|e| {
            let error_response =
                serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    Ok(
        Json(UserBody {
            user: User {
                user_id: user_id,
                name: req.user.name.clone(),
            },
        })
    )
}

pub fn router() -> Router {
    Router::new().route("/users", get(get_users).post(post_users))
}
