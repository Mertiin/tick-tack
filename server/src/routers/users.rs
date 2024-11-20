use axum::extract::Path;
use axum::http::StatusCode;
use axum::{ middleware, Extension, Json };
use axum::{ response::IntoResponse, routing::get, Router };
use log::error;
use uuid::Uuid;
use sqlx::{ query_as, query };
use pwhash::bcrypt;

use crate::auth::access_token::encode_jwt;
use crate::auth::authorization_middleware::{ auth, AuthExtension };
use crate::db::auth::create_refresh_token;
use crate::db::organization::{ self, get_orgs_by_user_id };
use crate::db::user::create_user;
use crate::models::user::User;
use crate::AppState;

#[derive(serde::Serialize, serde::Deserialize)]
struct ReturnUsers<T> {
    count: usize,
    users: Vec<T>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct ReturnUser {
    id: Uuid,
    email: String,
}

#[allow(non_snake_case)]
#[derive(serde::Serialize, serde::Deserialize)]
struct CreateReturnUser {
    id: Uuid,
    email: String,
    refreshToken: String,
    accessToken: String,
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
            id: user.id,
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

#[derive(serde::Deserialize, Clone)]
struct NewUser {
    email: String,
    password: String,
}

async fn post_users(
    ctx: Extension<AppState>,
    Json(req): Json<NewUser>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let hashed_password = bcrypt::hash(req.password).map_err(|_e| {
        let error_response =
            serde_json::json!({
        "status": "error",
        "message": "Failed to hash password",
    });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let new_user = match create_user(&req.email, &hashed_password, &ctx).await {
        Ok(id) => {
            let access_token = match encode_jwt(id.to_string(), req.email.clone()) {
                Ok(token) => token,
                Err(e) => {
                    let error_response =
                        serde_json::json!({
                        "status": "error",
                        "message": format!("JWT error: {}", e),
                    });
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
                }
            };

            let refresh_token = match create_refresh_token(id, &ctx).await {
                Ok(token) => token,
                Err(e) => {
                    let error_response =
                        serde_json::json!({
                        "status": "error",
                        "message": format!("Failed to create refresh token"),
                    });

                    return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
                }
            };

            CreateReturnUser {
                id: id,
                email: req.email,
                refreshToken: refresh_token,
                accessToken: access_token,
            }
        }
        Err(e) => {
            error!("Failed to insert user: {:?}", e);
            let error_response = match &e {
                sqlx::Error::Database(db_err) if db_err.code().as_deref() == Some("23505") => {
                    (
                        StatusCode::CONFLICT,
                        Json(
                            serde_json::json!({
                        "status": "error",
                        "message": "Email already exists",
                    })
                        ),
                    )
                }
                _ => {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(
                            serde_json::json!({
                        "status": "error",
                        "message": "Failed to insert user",
                            })
                        ),
                    )
                }
            };
            return Err(error_response);
        }
    };

    Ok(Json(new_user))
}

async fn delete_user(
    ctx: Extension<AppState>,
    Path(user_id): Path<Uuid>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    query!(
        // language=PostgreSQL
        r#"delete from users where id = $1 returning id"#,
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

async fn me(
    ctx: Extension<AppState>,
    auth: Extension<AuthExtension>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let organizations = match get_orgs_by_user_id(&auth.user.id, &ctx).await {
        Ok(orgs) => orgs,
        Err(e) => {
            let error_response =
                serde_json::json!({
            "status": "error",
            "message": "Failed to get organizations",
        });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }
    };

    Ok(
        Json(
            serde_json::json!({
        "id": auth.user.id,
        "email": auth.user.email,
        "organizations": organizations,
    })
        )
    )
}

pub fn router() -> Router {
    Router::new()
        .route("/", get(get_users).post(post_users))
        .route("/:user_id", axum::routing::delete(delete_user))
        .route("/me", get(me).layer(middleware::from_fn(auth)))
}
