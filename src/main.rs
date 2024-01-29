use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use common::{dispatcher::Dispatcher, zmq::Server};
use common::{keys::FcitxKeySym, msgs};
use ctrlc::set_handler;
use nepali::nepali_dispatcher::NepaliDispatcher;
use pinyin::pinyin_dispatcher::PinyinDispatcher;

pub mod common;
pub mod nepali;
pub mod pinyin;

#[tokio::main]
async fn main() {
    if cfg!(feature = "fcp") {
        let dispatcher = PinyinDispatcher::new();
        event_loop("tcp://127.0.0.1:8087", dispatcher).await;
    } else if cfg!(feature = "fcn") {
        let dispatcher = NepaliDispatcher::new();
        event_loop("tcp://127.0.0.1:8089", dispatcher).await;
    } else {
        let dispatcher = PinyinDispatcher::new();
        event_loop("tcp://127.0.0.1:8085", dispatcher).await;
    }
}

async fn event_loop<D>(addr: &str, dispatcher: D)
where
    D: Dispatcher,
{
    let run = Arc::new(AtomicBool::new(true));
    let run_for_handler = run.clone();
    set_handler(move || {
        run_for_handler.store(false, Ordering::Release);
    })
    .expect("main: Failed to set signal handler.");

    let sock = Server::new(addr);
    while run.load(Ordering::SeqCst) {
        let res = sock.recv();
        if res.is_err() {
            println!("event_loop: Shutting down");
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
