use axum::Router;

pub mod matches;
pub mod users;

pub fn router() -> Router {
    Router::new().nest("/users", users::router()).nest("/matches", matches::router())
}
