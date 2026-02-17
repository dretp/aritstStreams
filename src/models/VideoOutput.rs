
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoOutput {
    pub path: Option<String>,
    pub playlist_url: Option<String>, // HLS/DASH
    pub thumbnail_url: Option<String>,
    pub duration_seconds: Option<f64>,
}
