use super::{
    message::Message,
    protocol::{request::Request, response::Response},
};

pub trait Workload {
    fn handle_workload(&self, request: Message<Request>) -> Message<Response>;
}
