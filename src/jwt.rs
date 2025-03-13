use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use axum::{http::StatusCode, response::IntoResponse, Json};

const SECRET: &[u8] = b"supersecretkey"; // Change this for production!

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // The subject (usually user ID or email)
    pub exp: usize,   // Expiry timestamp
}

// Generate a JWT token for a user
pub fn create_jwt(username: &str) -> Result<String, impl IntoResponse> {
    let expiration = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        + 3600; // Token expires in 1 hour

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration as usize,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET),
    )
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to generate token"))
}

// Validate and decode a JWT token
pub fn verify_jwt(token: &str) -> Result<Claims, impl IntoResponse> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(SECRET),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid or expired token"))
}
