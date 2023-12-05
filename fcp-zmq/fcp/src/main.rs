use quick_protobuf::{BytesReader, MessageRead};

use crate::ims_send::FcitxEvent;

pub mod ims_recv;
pub mod ims_send;

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
        req.send("Hi from FCP", 0).expect("Failed to send to IMS.");
    }
}
