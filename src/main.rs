use dispatcher::Dispatcher;
use ims::Sub;
use keys::FcitxKeySym;

pub mod candidate;
pub mod candidate_service;
pub mod cloud_pinyin_client;
pub mod dispatcher;
pub mod ims;
pub mod keys;
pub mod msgs;
pub mod number_service;
pub mod preedit_service;
pub mod symbol_service;

#[tokio::main]
async fn main() {
    let sub = Sub::new("tcp://127.0.0.1:8085");
    let dispatcher = Dispatcher::new();
    loop {
        let event: msgs::KeyEvent = sub.recv();
        let key_u32 = event.key;
        let key = FcitxKeySym::from_u32(key_u32);
        if key.is_none() {
            continue;
        }
        _ = dispatcher.on_input(key.unwrap()).await;
    }
}
