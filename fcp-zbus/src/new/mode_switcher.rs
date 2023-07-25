use std::sync::{Arc, Mutex};

use crate::keys::Key;

pub struct ModeSwitcher {
    mode: Arc<Mutex<Mode>>,
}

impl ModeSwitcher {
    pub async fn process_key_event(
        &self,
        keyval: u32,
        keycode: u32,
        state: u32,
    ) -> ModeSwitcherReturn {
        // let bi = format!("{state:b}");
        // println!("keyval: {keyval}, keycode: {keycode}, state: {bi}");

        // State flags
        let is_release = self.get_kth_bit(state, 30);
        let is_ctrl = self.get_kth_bit(state, 2);
        let mut should_reset = false;

        if is_ctrl && is_release {
            let prev_mode = self.mode();
            if prev_mode == Mode::English {
                self.set_mode(Mode::Pinyin);
            } else {
                self.set_mode(Mode::English);
            }
            if prev_mode == Mode::Pinyin {
                // If *now* we are in English mode, reset the engine.
                should_reset = true;
            }
        }
        if is_ctrl && !is_release {
            // User control like ctrl+v that has nothing to do with us.
            return ModeSwitcherReturn::Done(false);
        }

        if self.mode() == Mode::English || is_release {
            return ModeSwitcherReturn::Done(false);
        }

        let maybe_key = Key::from_u32(keyval);
        if maybe_key.is_none() {
            return ModeSwitcherReturn::Done(false); // We don't handle anything outside of key.
        }
        let key = maybe_key.expect("maybe_key is None but it shouldn't.");
        return ModeSwitcherReturn::Continue(key, should_reset);
    }

    fn mode(&self) -> Mode {
        *self.mode.lock().expect("Failed to lock mode.")
    }

    fn set_mode(&self, val: Mode) {
        let mut mode = self.mode.lock().expect("Failed to lock mode.");
        *mode = val;
    }

    fn get_kth_bit(&self, n: u32, k: u32) -> bool {
        (n & (1 << k)) >> k == 1
    }
}


#[derive(Clone, Copy)]
pub enum ModeSwitcherReturn {
    Continue(Key, bool),
    Done(bool),
}

impl ModeSwitcherReturn {
    pub fn get_continue_data(&self) -> Option<(Key, bool)> {
        match self {
            ModeSwitcherReturn::Continue(key, should_reset) => {
                Some((key.clone(), should_reset.clone()))
            }
            ModeSwitcherReturn::Done(_) => None,
        }
    }

    pub fn get_done_data(&self) -> Option<bool> {
        match self {
            ModeSwitcherReturn::Continue(_, _) => None,
            ModeSwitcherReturn::Done(has_handled) => Some(has_handled.clone()),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Mode {
    English,
    Pinyin,
}