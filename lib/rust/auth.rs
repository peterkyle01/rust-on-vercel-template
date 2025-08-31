use anyhow::{anyhow, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct User {
    #[ts(type = "string")]
    pub id: Uuid,
    pub email: String,
    pub username: String,
    #[ts(type = "string")]
    pub created_at: DateTime<Utc>,
    #[ts(type = "string")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CreateUserRequest {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user id
    pub email: String,
    pub exp: usize, // expiration time
    pub iat: usize, // issued at
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ApiError {
    pub message: String,
    pub code: u16,
}

pub fn hash_password(password: &str) -> Result<String> {
    let hashed = hash(password, DEFAULT_COST)?;
    Ok(hashed)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool> {
    let is_valid = verify(password, hash)?;
    Ok(is_valid)
}

pub fn create_jwt(user_id: &Uuid, email: &str) -> Result<String> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::hours(24)).timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        email: email.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_ref()),
    )?;

    Ok(token)
}

pub fn verify_jwt(token: &str) -> Result<Claims> {
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_ref()),
        &Validation::new(Algorithm::HS256),
    )?;

    Ok(token_data.claims)
}

pub fn extract_bearer_token(auth_header: &str) -> Result<&str> {
    if auth_header.starts_with("Bearer ") {
        Ok(&auth_header[7..])
    } else {
        Err(anyhow!("Invalid authorization header format"))
    }
}
