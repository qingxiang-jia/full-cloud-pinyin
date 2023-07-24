use std::sync::{Mutex, Arc};

#[derive(Clone, Copy)]
enum Mode {
    English,
    Pinyin
}

pub struct ModeSwitcher {
    mode: Arc<Mutex<Mode>>
}

impl ModeSwitcher {
    pub async fn process_key_event(&self, keyval: u32, keycode: u32, state: u32) -> bool {
        !unimplemented!()
    }

    fn mode(&self) -> Mode {
        *self.mode.lock().expect("Failed to lock mode.")
    }

    fn set_mode(&self, val: Mode) {
        let mut mode = self.mode.lock().expect("Failed to lock mode.");
        *mode = val;
    }
}