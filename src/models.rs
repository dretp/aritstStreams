use serde::{Deserialize, Serialize};
use sqlx::FromRow;


#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub public_id: String,
    pub first_name: String,
    pub last_name: String,
}