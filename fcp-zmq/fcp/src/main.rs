use std::borrow::Cow;

use ims_recv::{CommandToFcitx, CommitText};
use quick_protobuf::{BytesReader, MessageRead, Writer, MessageWrite};

use crate::ims_send::FcitxEvent;

pub mod ims_recv;
pub mod ims_send;
pub mod ims;

fn main() {
    let ctx = zmq::Context::new();
    let sub = ctx
        .socket(zmq::SUB)
        .expect("Failed to create a SUB socket.");
    sub.connect("tcp://127.0.0.1:8085")
        .expect("Failed to connect to the publisher address.");
    sub.set_subscribe(b"").expect("Failed to subscribe to any.");
    let req = ctx
        .socket(zmq::REQ)
        .expect("Failed to create a REQ socket.");
    req.connect("tcp://127.0.0.1:8086")
        .expect("Failed to connect to the reply address.");
    loop {
        let data = sub
            .recv_msg(0)
            .expect("Failed to receive published message.");
        unsafe {
            let bytes = std::slice::from_raw_parts(data.as_ptr(), data.len());
            let mut reader = BytesReader::from_bytes(&bytes);
            let fe = FcitxEvent::from_reader(&mut reader, bytes);
            println!("event is: {:#?}", &fe);
        }
        let cmd = CommitText {
            text: Cow::from("text to commit")
        };
        let cmd_container = CommandToFcitx {
            command: ims_recv::mod_CommandToFcitx::OneOfcommand::commit_text(cmd)
        };
        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);
        CommandToFcitx::default().write_message(&mut writer).expect("Failed to write message.");
        cmd_container.write_message(&mut writer).expect("Failed to write message.");
        
        req.send(out, 0).expect("Failed to send to IMS.");
        _ = req.recv_msg(0).expect("Failed to receive reply of REQ.");
    }
}
