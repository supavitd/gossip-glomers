use serde::{Deserialize, Serialize};

use super::message_type::MessageType;

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename = "init")]
pub struct Init {
    pub node_id: String,
    pub node_ids: Vec<String>,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", rename = "init_ok")]
pub struct InitOk {}
