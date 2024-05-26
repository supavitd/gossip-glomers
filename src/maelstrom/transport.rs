mod local;

pub trait Transport {
    type Msg;

    fn send_message(&self, msg: Self::Msg);
    fn receive_message(&self) -> Self::Msg;
}
