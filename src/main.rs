mod echo;
mod maelstrom;

use echo::*;
use maelstrom::{message::Message, node::Node, transport::Transport};

use crate::maelstrom::protocol::{
    init::{Init, InitOk},
    request::Request,
    response::Response,
};

fn main() {
    let node = init().unwrap();

    dbg!("{?:}", &node);

    listen_for_messages(&node);
}

fn init() -> serde_json::Result<Node> {
    // let input = r#"
    // {
    //     "type": "init",
    //     "msg_id": 1,
    //     "node_id": "n1",
    //     "node_ids": ["n1", "n2", "n3"]
    // }
    // "#;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let init_msg: Message<Request<Init>> = serde_json::from_str(&input)?;

    // let request: Request<Init> = serde_json::from_str(input)?;
    // dbg!("{?:}", &request);
    //
    // println!("{}", serde_json::to_string_pretty(&request).unwrap());
    //
    // let init_msg: Init = serde_json::from_str(input)?;
    //
    // dbg!("{?:}", &init_msg);

    let node = Node {
        id: init_msg.body.data.node_id.clone(),
    };

    reply_init_ok(&node, &init_msg);

    // TODO: Prepare to handle requests. Init runner etc.

    Ok(node)
}

fn listen_for_messages(node: &Node) {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let Ok(msg) = serde_json::from_str::<Message<Request<Echo>>>(&input) else {
            dbg!("Invalid incoming message {}", &input);
            continue;
        };

        let resp = Response {
            in_reply_to: msg.body.msg_id,
            data: EchoOk {
                echo: msg.body.data.echo,
            },
        };

        let reply_msg = Message::new(node, msg.src, resp);
        println!("{}", serde_json::to_string(&reply_msg).unwrap());
    }
}

fn reply_init_ok(node: &Node, msg: &Message<Request<Init>>) {
    let resp = Response {
        in_reply_to: msg.body.msg_id,
        data: InitOk {},
    };

    let reply_msg = Message::<Response<InitOk>>::new(node, msg.src, resp);
    dbg!("{?:}", &reply_msg);

    println!("{}", serde_json::to_string(&reply_msg).unwrap());
}
