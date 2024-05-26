use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Response<T> {
    pub in_reply_to: u32,

    #[serde(flatten)]
    pub data: T,
}
