use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use super::message_type::MessageType;

#[derive(Deserialize, Debug, Serialize)]
pub struct Request {
    pub msg_id: u32,

    #[serde(flatten)]
    pub payload: ReqPayload,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReqPayload {
    Init {
        node_id: String,
        node_ids: Vec<String>,
    },
    Echo {
        echo: String,
    },
    Generate {},
}
