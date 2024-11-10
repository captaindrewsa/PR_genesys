use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MirrorJson {
    message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MirrorJasonRespone {
    message: String,
    message_response: String,
}

pub async fn mirror_json(Json(body): Json<MirrorJson>) -> Json<MirrorJasonRespone> {
    Json(MirrorJasonRespone {
        message: body.message,
        message_response: "Something from me".to_owned(),
    })
}
