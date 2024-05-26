use crate::maelstrom::node::Node;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message<'a, T> {
    pub src: &'a str,
    pub dest: &'a str,
    pub body: T,
}

impl<'a, T> Message<'a, T> {
    pub fn new(src: &'a Node, dest: &'a str, body: T) -> Self {
        Message {
            src: &src.id,
            dest,
            body,
        }
    }
}
