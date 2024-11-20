use axum::Router;

pub mod users;
pub mod auth;
pub mod organization;

pub fn router() -> Router {
    Router::new()
        .nest("/users", users::router())
        .nest("/auth", auth::router())
        .nest("/organizations", organization::router())
}
