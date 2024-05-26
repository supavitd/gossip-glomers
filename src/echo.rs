use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename = "echo")]
pub struct Echo<'a> {
    pub echo: &'a str,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename = "echo_ok")]
pub struct EchoOk<'a> {
    pub echo: &'a str,
}
