use jsonwebtoken::{ DecodingKey, EncodingKey };

use std::env;

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new() -> Self {
        let jwt_secret = env::var("JWT_SECRET").unwrap();
        let secret = jwt_secret.as_bytes();
        Self {
            encoding: EncodingKey::from_secret(&secret),
            decoding: DecodingKey::from_secret(&secret),
        }
    }
}
