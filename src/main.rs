use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use ctrlc::set_handler;
use dispatcher::Dispatcher;
use keys::FcitxKeySym;
use zmq::Server;

pub mod candidate;
pub mod candidate_service;
pub mod cloud_pinyin_client;
pub mod dispatcher;
pub mod keys;
pub mod msgs;
pub mod number_service;
pub mod path_util;
pub mod preedit_service;
pub mod symbol_service;
pub mod user_dict;
pub mod zmq;

#[tokio::main]
async fn main() {
    let run = Arc::new(AtomicBool::new(true));
    let run_for_handler = run.clone();
    set_handler(move || {
        run_for_handler.store(false, Ordering::Release);
    })
    .expect("main: Failed to set signal handler.");

    let sock = Server::new("tcp://127.0.0.1:8085");
    let dispatcher = Dispatcher::new();
    while run.load(Ordering::SeqCst) {
        let res = sock.recv();
        if res.is_err() {
            println!("main: Shutting down");
            break; // Most likely we received ctrl+c, so it's okay.
        }
        let event: msgs::KeyEvent = res.unwrap();
        let key_u32 = event.key;
        let key = FcitxKeySym::from_u32(key_u32);
        if key.is_none() {
            continue;
        }
        dispatcher.on_input(key.unwrap(), &sock).await;
    }
}
