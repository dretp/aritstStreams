use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VideoInput {
    Upload {
        path: String,
    },
    Url {
        url: String,
    },
    Stream {
        stream_id: String,
    },
}
