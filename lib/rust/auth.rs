use anyhow::{anyhow, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sqlx::{PgPool, Row};
use std::env;
use uuid::Uuid;

use crate::{Claims, CreateUserRequest, User, UserRepository, UserWithPassword};

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

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create_user(&self, request: CreateUserRequest) -> Result<User> {
        // Check if user already exists
        let existing_user = sqlx::query("SELECT id FROM users WHERE email = $1 OR username = $2")
            .bind(&request.email)
            .bind(&request.username)
            .fetch_optional(&self.pool)
            .await?;

        if existing_user.is_some() {
            return Err(anyhow!("User with this email or username already exists"));
        }

        // Hash password
        let password_hash = hash_password(&request.password)?;
        let user_id = Uuid::new_v4();

        // Create user
        let user_row = sqlx::query(
            r#"
            INSERT INTO users (id, email, username, password_hash, created_at, updated_at)
            VALUES ($1, $2, $3, $4, NOW(), NOW())
            RETURNING id, email, username, created_at, updated_at
            "#,
        )
        .bind(&user_id)
        .bind(&request.email)
        .bind(&request.username)
        .bind(&password_hash)
        .fetch_one(&self.pool)
        .await?;

        Ok(User {
            id: user_row.get("id"),
            email: user_row.get("email"),
            username: user_row.get("username"),
            created_at: user_row.get("created_at"),
            updated_at: user_row.get("updated_at"),
        })
    }

    pub async fn authenticate_user(&self, email: &str, password: &str) -> Result<User> {
        let user_row = sqlx::query_as::<_, UserWithPassword>(
            "SELECT id, email, username, password_hash, created_at, updated_at FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        let user_data = user_row.ok_or_else(|| anyhow!("Invalid credentials"))?;

        if !verify_password(password, &user_data.password_hash)? {
            return Err(anyhow!("Invalid credentials"));
        }

        Ok(User {
            id: user_data.id,
            email: user_data.email,
            username: user_data.username,
            created_at: user_data.created_at,
            updated_at: user_data.updated_at,
        })
    }

    pub async fn get_user_by_id(&self, user_id: &Uuid) -> Result<Option<User>> {
        let user_row = sqlx::query(
            "SELECT id, email, username, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = user_row {
            Ok(Some(User {
                id: row.get("id"),
                email: row.get("email"),
                username: row.get("username"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<Option<User>> {
        let user_row = sqlx::query(
            "SELECT id, email, username, created_at, updated_at FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;

        if let Some(row) = user_row {
            Ok(Some(User {
                id: row.get("id"),
                email: row.get("email"),
                username: row.get("username"),
                created_at: row.get("created_at"),
                updated_at: row.get("updated_at"),
            }))
        } else {
            Ok(None)
        }
    }
}
