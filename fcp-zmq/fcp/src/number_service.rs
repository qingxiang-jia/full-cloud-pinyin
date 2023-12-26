use std::sync::Arc;
use std::sync::Mutex;

use crate::ims::Req;
use crate::keys::FcitxKeySym;

pub struct NumberService {
    zmq: Arc<Mutex<Req>>,
}

impl NumberService {
    pub fn new(ibus: Arc<Mutex<Req>>) -> NumberService {
        NumberService { zmq: ibus }
    }

    pub fn handle_number(&self, key: FcitxKeySym) {
        let n = key
            .to_usize()
            .expect("This key cannot be converted to a usize.");

        let text = n.to_string();

        self.zmq
            .lock()
            .expect("handle_number: Failed to lock zmq.")
            .commit_text(&text);
    }
}
