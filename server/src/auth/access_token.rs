use axum::http::StatusCode;
use chrono::{ Duration, Utc };
use jsonwebtoken::{ encode, decode, Header, TokenData, Validation };
use serde::{ Deserialize, Serialize };

use crate::auth::keys::Keys;

#[derive(Deserialize, Serialize)]
pub struct Claims {
    pub user_id: String,
    pub email: String,
    pub exp: usize, // Expiry time of the token
    pub iat: usize, // Issued at time of the token
}

pub fn encode_jwt(user_id: String, email: String) -> Result<String, StatusCode> {
    let now = Utc::now();
    let expire: chrono::TimeDelta = Duration::minutes(15);
    let exp: usize = (now + expire).timestamp() as usize;
    let iat: usize = now.timestamp() as usize;
    let claim = Claims { iat, exp, user_id, email };

    encode(&Header::default(), &claim, &Keys::new().encoding).map_err(
        |_| StatusCode::INTERNAL_SERVER_ERROR
    )
}

pub fn decode_jwt(jwt_token: String) -> Result<TokenData<Claims>, StatusCode> {
    let result: Result<TokenData<Claims>, StatusCode> = decode(
        &jwt_token,
        &Keys::new().decoding,
        &Validation::default()
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR);
    result
}
