use axum::Json;
use serde::Deserialize;
use tokio::task;

use crate::models::{VideoJob, VideoInput};
use crate::services::gs_streamer_pipeline::GStreamerPipelineService;

#[derive(Deserialize)]
pub struct TestRequest {
    pub input: VideoInput,
    pub output_dir: String,
}

pub async fn test_pipeline(
    Json(req): Json<TestRequest>,
) -> Json<VideoJob> {
    let mut job = VideoJob::new(req.input, req.output_dir);

    let mut job_clone = job.clone();
    task::spawn_blocking(move || {
        let _ = GStreamerPipelineService::run(&mut job_clone);
    });

    Json(job)
}
