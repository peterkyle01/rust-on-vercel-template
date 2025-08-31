// This is a shared module for your structs and common functions.
pub mod auth;
pub mod database;
pub mod repository;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

// Re-export commonly used types
pub use auth::{ApiError, AuthResponse, CreateUserRequest, LoginRequest, User};
pub use database::{create_pool, run_migrations};
pub use repository::UserRepository;

#[derive(TS, Serialize, Deserialize)]
#[ts(export)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub price: f64,
}
