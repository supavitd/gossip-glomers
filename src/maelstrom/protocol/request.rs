use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use super::message_type::MessageType;

#[derive(Deserialize, Debug, Serialize)]
pub struct Request<T> {
    pub msg_id: u32,

    #[serde(flatten)]
    pub data: T,
}
