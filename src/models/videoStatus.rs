use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VideoStatus {
    Pending,
    Processing,
    Completed,
    Failed {
        reason: String,
    },
}