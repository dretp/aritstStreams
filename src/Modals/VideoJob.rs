use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::SystemTime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoJob {
    pub id: Uuid,

    // Input
    pub input: VideoInput,

    // Output
    pub output: VideoOutput,

    // State
    pub status: VideoStatus,

    // Metadata
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}
