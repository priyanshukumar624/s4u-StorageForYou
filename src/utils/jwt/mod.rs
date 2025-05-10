// src/utils/jwt.rs

use jsonwebtoken::{encode, decode, EncodingKey, DecodingKey, Header, Validation, TokenData, errors::Error};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration};

const SECRET_KEY: &[u8] = b"your_secret_key_here";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id or email
    pub exp: usize,  // expiration timestamp
}

pub fn create_jwt(user_id: &str) -> Result<String, Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::minutes(60))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(SECRET_KEY))
}

pub fn validate_jwt(token: &str) -> Result<TokenData<Claims>, Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET_KEY),
        &Validation::default(),
    )
}
