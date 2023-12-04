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
        println!("received length: {}", data.len());
    }
}
