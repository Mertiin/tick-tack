use jsonwebtoken::{ DecodingKey, EncodingKey };

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new() -> Self {
        let secret = dotenv!("JWT_SECRET").as_bytes();
        Self {
            encoding: EncodingKey::from_secret(&secret),
            decoding: DecodingKey::from_secret(&secret),
        }
    }
}
