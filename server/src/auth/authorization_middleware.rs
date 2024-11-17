use axum::{ extract::Request, middleware::Next, response::Response };
use http::StatusCode;

use crate::{ db::user::get_user_by_email, models::user::User, state::AppState };

use super::access_token::decode_jwt;

#[derive(Clone)]
pub struct AuthExtension {
    pub user: User,
}

pub async fn auth(mut req: Request, next: Next) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok());
    let ctx = req.extensions().get::<AppState>().unwrap();

    if let Some(header) = auth_header {
        if header.starts_with("Bearer ") {
            let jwt_token = &header[7..];
            // Now you can use jwt_token
            match decode_jwt(jwt_token.to_string()) {
                Ok(token) => {
                    match get_user_by_email(&token.claims.email, &ctx).await {
                        Ok(user) => {
                            req.extensions_mut().insert(AuthExtension { user });
                            return Ok(next.run(req).await);
                        }
                        Err(_) => {
                            return Err(StatusCode::UNAUTHORIZED);
                        }
                    }
                }
                Err(_) => {
                    return Err(StatusCode::UNAUTHORIZED);
                }
            }
        } else {
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        return Err(StatusCode::UNAUTHORIZED);
    }
}
