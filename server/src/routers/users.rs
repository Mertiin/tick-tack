use axum::extract::Path;
use axum::http::StatusCode;
use axum::{ Extension, Json };
use axum::{ response::IntoResponse, routing::get, Router };
use log::error;
use uuid::Uuid;
use sqlx::{ query_as, query_scalar, query };
use pwhash::bcrypt;

use crate::models::user::User;
use crate::AppState;

#[derive(serde::Serialize, serde::Deserialize)]
struct UserBody<T> {
    user: T,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ReturnUsers<T> {
    count: usize,
    users: Vec<T>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ReturnUser {
    user_id: Uuid,
    email: String,
}

#[axum::debug_handler]
async fn get_users(
    ctx: Extension<AppState>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let users = query_as::<_, User>(r#"SELECT * FROM users"#)
        .fetch_all(&ctx.db).await
        .map_err(|_e| {
            let error_response =
                serde_json::json!({
            "status": "error",
            "message": "Failed to fetch users",
        });
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    let users: Vec<ReturnUser> = users
        .into_iter()
        .map(|user| ReturnUser {
            user_id: user.user_id,
            email: user.email,
        })
        .collect();

    let json_response =
        serde_json::json!({
        "count": users.len(),
        "users": users
    });

    Ok(Json(json_response))
}

#[derive(serde::Deserialize)]
struct NewUser {
    email: String,
    password: String,
}

async fn post_users(
    ctx: Extension<AppState>,
    Json(req): Json<UserBody<NewUser>>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let hashed_password = bcrypt::hash(req.user.password).map_err(|_e| {
        let error_response =
            serde_json::json!({
        "status": "error",
        "message": "Failed to hash password",
    });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let user_id = query_scalar!(
        // language=PostgreSQL
        r#"insert into users (email, password) values ($1, $2) returning user_id"#,
        req.user.email,
        hashed_password
    )
        .fetch_one(&ctx.db).await
        .map_err(|e| {
            error!("Failed to insert user: {:?}", e);
            let error_response;
            if let sqlx::Error::Database(db_err) = &e {
                if db_err.code().as_deref() == Some("23505") {
                    error_response =
                        serde_json::json!({
                "status": "error",
                "message": "Email already exists",
            });
                } else {
                    error_response =
                        serde_json::json!({
                "status": "error",
                "message": "Failed to insert user",
            });
                }
            } else {
                error_response =
                    serde_json::json!({
            "status": "error",
            "message": "Failed to insert user",
        });
            }
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        })?;

    Ok(
        Json(UserBody {
            user: ReturnUser {
                user_id: user_id,
                email: req.user.email,
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
        .map_err(|_e| {
            let error_response =
                serde_json::json!({
            "status": "error",
            "message": "Failed to delete user",
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
