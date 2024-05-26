use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(tag = "type", rename = "error")]
pub struct Error {
    code: ErrorCode,
    text: String,
}

#[derive(Serialize, Debug)]
pub enum ErrorCode {
    Timeout,
    NodeNotFound,
    NotSupported,
    TemporarilyUnavailable,
    MalformedRequest,
    Crash,
    Abort,
    KeyDoesNotExist,
    PreconditionFailed,
    TxnConflict,
}
