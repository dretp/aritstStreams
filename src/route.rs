use axum::{routing::post, Router};

use crate::controllers::videocontroller::test_pipeline;
pub fn routes() -> Router {
     Router::new()
        .route("/video/test", post(test_pipeline))
}
