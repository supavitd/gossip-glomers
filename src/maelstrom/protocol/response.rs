use serde::{Deserialize, Serialize};

use super::error::ErrorCode;

#[derive(Deserialize, Serialize, Debug)]
pub struct Response {
    pub in_reply_to: u32,

    #[serde(flatten)]
    pub payload: ResPayload,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ResPayload {
    InitOk,
    Error { code: ErrorCode, text: String },
    EchoOk { echo: String },
    GenerateOk { id: String },
}
