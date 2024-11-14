use axum::extract::Path;
use axum::http::StatusCode;
use axum::{ Extension, Json };
use axum::{ response::IntoResponse, routing::get, Router };
use uuid::Uuid;
use sqlx::{ query_as, query_scalar, query, FromRow };

use crate::AppState;

#[derive(serde::Serialize, serde::Deserialize)]
struct UserBody<T> {
    user: T,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, FromRow)]
struct User {
    user_id: uuid::Uuid,
    name: String,
}

#[axum::debug_handler]
async fn get_users(
    ctx: Extension<AppState>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let users = query_as::<_, User>(r#"SELECT * FROM users"#)
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

async fn post_users(
    ctx: Extension<AppState>,
    Json(req): Json<UserBody<NewUser>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = query_scalar!(
        // language=PostgreSQL
        r#"insert into users (name) values ($1) returning user_id"#,
        req.user.name
    )
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
                name: req.user.name,
            },
        })
    )
}

async fn delete_user(
    ctx: Extension<AppState>,
    Path(user_id): Path<Uuid>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    query!(
        // language=PostgreSQL
        r#"delete from users where user_id = $1 returning user_id"#,
        user_id
    )
        .fetch_one(&ctx.db).await
        .map_err(|e| {
            let error_response =
                serde_json::json!({
            "status": "error",
            "message": format!("Database error: {}", e),
        });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let json_response = serde_json::json!({
        "status": "ok",
    });

    Ok(Json(json_response))
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_users).post(post_users))
        .route("/:user_id", axum::routing::delete(delete_user))
}
