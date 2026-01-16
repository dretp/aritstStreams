use serde::{Deserialize, Serialize};
use sqlx::FromRow;

/// Represents a User entity.
/// This struct can be used for database operations and JSON serialization.
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub public_id: String,
    pub first_name: String,
    pub last_name: String,
}