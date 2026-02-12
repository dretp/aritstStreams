use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VideoStatus {
    Pending,
    Processing,
    Completed,
    Failed {
        reason: String,
    },
}