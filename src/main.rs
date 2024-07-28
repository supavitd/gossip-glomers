mod maelstrom;

use std::sync::atomic::{AtomicU32, Ordering};

use maelstrom::{message::Message, node::Node, transport::Transport};

use crate::maelstrom::protocol::{
    request::{ReqPayload, Request},
    response::{ResPayload, Response},
};

static SEQ_ID: AtomicU32 = AtomicU32::new(0);

fn main() {
    let node = init().unwrap();

    dbg!("{?:}", &node);

    listen_for_messages(&node);
}

fn init() -> serde_json::Result<Node> {
    // let input = r#"
    // {
    //     "src": "test",
    //     "dest": "n1",
    //     "body": {
    //         "type": "init",
    //         "msg_id": 1,
    //         "node_id": "n1",
    //         "node_ids": ["n1", "n2", "n3"]
    //     }
    // }
    // "#;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    let init_msg: Message<Request> = serde_json::from_str(&input)?;
    dbg!("{?:}", &init_msg);

    let ReqPayload::Init { node_id, .. } = init_msg.body.payload else {
        panic!("Did not receive a valid Init request.");
    };

    let node = Node { id: node_id };

    reply_init_ok(&node, init_msg.src, init_msg.body.msg_id);

    // TODO: Prepare to handle requests. Init runner etc.

    Ok(node)
}

fn generate_id(node: &Node) -> String {
    let num = SEQ_ID.fetch_add(1, Ordering::Relaxed);
    let id = format!("{}-{}", &node.id, num);
    id
}

fn listen_for_messages(node: &Node) {
    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        let Ok(msg) = serde_json::from_str::<Message<Request>>(&input) else {
            dbg!("Invalid incoming message {}", &input);
            continue;
        };

        let payload = match msg.body.payload {
            ReqPayload::Echo { echo } => ResPayload::EchoOk { echo },
            ReqPayload::Generate {} => ResPayload::GenerateOk {
                id: format!("{}{}", node.id, msg.body.msg_id),
            },
            _ => unimplemented!(),
        };

        let reply_msg = Message::new(
            node,
            msg.src,
            Response {
                in_reply_to: msg.body.msg_id,
                payload,
            },
        );
        println!("{}", serde_json::to_string(&reply_msg).unwrap());
    }
}

fn reply_init_ok(node: &Node, dest: &str, in_reply_to: u32) {
    let resp = Response {
        in_reply_to,
        payload: ResPayload::InitOk,
    };

    let reply_msg = Message::<Response>::new(node, dest, resp);
    dbg!("{?:}", &reply_msg);

    println!("{}", serde_json::to_string(&reply_msg).unwrap());
}
