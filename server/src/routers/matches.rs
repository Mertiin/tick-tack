use axum::http::StatusCode;
use axum::{ Extension, Json };
use axum::{ response::IntoResponse, routing::get, Router };
use sqlx::{ query_as, FromRow };

use crate::AppState;

#[derive(Debug, serde::Serialize, serde::Deserialize, FromRow)]
struct Match {
    user_id: uuid::Uuid,
}

#[axum::debug_handler]
async fn get_matches(
    ctx: Extension<AppState>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let users = query_as::<_, Match>(r#"SELECT * FROM matches"#)
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
        "matches": users
    });

    Ok(Json(json_response))
}

pub fn router() -> Router {
    Router::new().route("/", get(get_matches))
}
