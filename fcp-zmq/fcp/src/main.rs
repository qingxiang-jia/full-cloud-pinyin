use quick_protobuf::{MessageRead, BytesReader};

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
    }
}
