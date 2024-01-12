use std::sync::Arc;

use std::sync::Mutex;

use crate::common::keys::FcitxKeySym;
use crate::common::zmq::Client;

pub struct SymbolService {
    pub(crate) zmq: Arc<Mutex<Client>>,
}

impl SymbolService {
    pub fn new(ibus: Arc<Mutex<Client>>) -> SymbolService {
        SymbolService { zmq: ibus }
    }

    pub fn handle_symbol(&self, key: FcitxKeySym) {
        let fw_puctuation = key
            .to_chinese()
            .expect("This key cannot be converted to fullwidth string.");

        self.zmq
            .lock()
            .expect("handle_symbol: Failed to lock zmq.")
            .commit_text(&fw_puctuation);
    }
}
