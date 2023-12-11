use std::sync::Arc;

use std::sync::Mutex;

use crate::ims::Req;
use crate::keys::Key;

pub struct SymbolService {
    pub(crate) zmq: Arc<Mutex<Req>>,
}

impl SymbolService {
    pub fn new(ibus: Arc<Mutex<Req>>) -> SymbolService {
        SymbolService { zmq: ibus }
    }

    pub async fn handle_symbol(&self, key: Key) {
        let fw_puctuation = key
            .to_full_width_string()
            .expect("This key cannot be converted to fullwidth string.");

        self.zmq
            .lock()
            .expect("handle_symbol: Failed to lock zmq.")
            .commit_text(&fw_puctuation);
    }
}
