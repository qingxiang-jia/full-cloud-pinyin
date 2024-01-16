use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use common::{dispatcher::Dispatcher, zmq::Server};
use common::{keys::FcitxKeySym, msgs};
use ctrlc::set_handler;
use pinyin::pinyin_dispatcher::PinyinDispatcher;

pub mod common;
pub mod nepali;
pub mod pinyin;

#[tokio::main]
async fn main() {
    let run = Arc::new(AtomicBool::new(true));
    let run_for_handler = run.clone();
    set_handler(move || {
        run_for_handler.store(false, Ordering::Release);
    })
    .expect("main: Failed to set signal handler.");

    let sock = Server::new("tcp://127.0.0.1:8085");
    let dispatcher = PinyinDispatcher::new();
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
