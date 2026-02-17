use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::time::SystemTime;

use super::{VideoInput, VideoStatus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoJob {
    pub id: Uuid,
    pub input: VideoInput,
    pub output_dir: String,
    pub status: VideoStatus,
    pub created_at: SystemTime,
    pub updated_at: SystemTime,
}

impl VideoJob {
    pub fn new(input: VideoInput, output_dir: String) -> Self {
        let now = SystemTime::now();

        Self {
            id: Uuid::new_v4(),
            input,
            output_dir,
            status: VideoStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mark_processing(&mut self) {
        self.status = VideoStatus::Processing;
        self.updated_at = SystemTime::now();
    }

    pub fn mark_completed(&mut self) {
        self.status = VideoStatus::Completed;
        self.updated_at = SystemTime::now();
    }

    pub fn mark_failed(&mut self, reason: impl Into<String>) {
        self.status = VideoStatus::Failed {
            reason: reason.into(),
        };
        self.updated_at = SystemTime::now();
    }
}
