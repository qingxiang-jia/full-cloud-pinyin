use std::sync::{Arc, Mutex};

use crate::common::{keys::FcitxKeySym, zmq::Client};

pub struct NumberService {
    zmq: Arc<Mutex<Client>>,
}

impl NumberService {
    pub fn new(zmq: Arc<Mutex<Client>>) -> NumberService {
        NumberService { zmq }
    }

    pub fn handle_number(&self, key: FcitxKeySym) {
        let text = match key {
            FcitxKeySym::Num0 => "०",
            FcitxKeySym::Num1 => "१",
            FcitxKeySym::Num2 => "२",
            FcitxKeySym::Num3 => "३",
            FcitxKeySym::Num4 => "४",
            FcitxKeySym::Num5 => "५",
            FcitxKeySym::Num6 => "६",
            FcitxKeySym::Num7 => "७",
            FcitxKeySym::Num8 => "८",
            FcitxKeySym::Num9 => "९",
            _ => "",
        };

        self.zmq
            .lock()
            .expect("handle_number: Failed to lock zmq.")
            .commit_text(&text);
    }
}
