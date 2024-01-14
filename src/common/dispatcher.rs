use super::{keys::FcitxKeySym, zmq::Server};

pub trait Dispatcher {
    fn new() -> Self;
    async fn on_input(&self, key: FcitxKeySym, sock: &Server);
}
