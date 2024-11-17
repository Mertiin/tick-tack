use axum::Router;

pub mod matches;
pub mod users;
pub mod auth;

pub fn router() -> Router {
    Router::new()
        .nest("/users", users::router())
        .nest("/matches", matches::router())
        .nest("/auth", auth::router())
}
