use axum::http::StatusCode;
use axum::routing::post;
use axum::{ middleware, Extension, Json };
use axum::{ response::IntoResponse, Router };
use log::debug;

use crate::auth::authorization_middleware::auth;
use crate::{
    auth::authorization_middleware::AuthExtension,
    db::organization::{ create_organization, CreateOrganization },
    state::AppState,
};

#[derive(serde::Deserialize)]
struct NewOrganization {
    name: String,
}

#[axum::debug_handler]
async fn post_organization(
    ctx: Extension<AppState>,
    auth: Extension<AuthExtension>,
    Json(req): Json<NewOrganization>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match
        create_organization(
            {
                let user = &auth.user;
                CreateOrganization {
                    name: req.name,
                    user_id: user.id,
                }
            },
            &ctx
        ).await
    {
        Ok(org_id) => {
            let json_response =
                serde_json::json!({
                "status": "ok",
                "organization_id": org_id,
            });
            Ok(Json(json_response))
        }
        Err(e) => {
            let error_response = if let Some(db_error) = e.as_database_error() {
                if db_error.code().as_deref() == Some("23505") {
                    (
                        StatusCode::CONFLICT,
                        Json(
                            serde_json::json!({
                        "message": "Organization already exists",
                    })
                        ),
                    )
                } else {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(
                            serde_json::json!({
                    "message": "Failed to create organization",
                })
                        ),
                    )
                }
            } else {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        serde_json::json!({
                    "message": "Failed to create organization",
                })
                    ),
                )
            };
            return Err(error_response);
        }
    }
}

pub fn router() -> Router {
    Router::new().route("/", post(post_organization).layer(middleware::from_fn(auth)))
}
