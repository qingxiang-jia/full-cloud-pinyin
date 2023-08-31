use std::sync::{Arc, Mutex};

use crate::keys::Key;

pub struct ModeSwitcher {
    mode: Arc<Mutex<Mode>>,
}

impl ModeSwitcher {
    pub fn new() -> ModeSwitcher {
        ModeSwitcher {
            mode: Arc::new(Mutex::new(Mode::Pinyin)),
        }
    }

    pub async fn process_key_event(
        &self,
        keyval: u32,
        keycode: u32,
        state: u32,
    ) -> ModeSwitcherReturn {
        // let bi = format!("{state:b}");
        // println!("keyval: {keyval}, keycode: {keycode}, state: {bi}");

        // State flags
        let is_shift = self.get_kth_bit(state, 0);
        let is_lock = self.get_kth_bit(state, 1);
        let is_ctrl = self.get_kth_bit(state, 2);
        let is_alt = self.get_kth_bit(state, 3);
        let is_mod2 = self.get_kth_bit(state, 4);
        let is_mod3 = self.get_kth_bit(state, 5);
        let is_mod4 = self.get_kth_bit(state, 6);
        let is_mod5 = self.get_kth_bit(state, 7);
        let is_btn1 = self.get_kth_bit(state, 8);
        let is_btn2 = self.get_kth_bit(state, 9);
        let is_btn3 = self.get_kth_bit(state, 10);
        let is_btn4 = self.get_kth_bit(state, 11);
        let is_btn5 = self.get_kth_bit(state, 12);
        let is_handled = self.get_kth_bit(state, 24);
        let is_ignored = self.get_kth_bit(state, 25);
        let is_super = self.get_kth_bit(state, 26);
        let is_hyper = self.get_kth_bit(state, 27);
        let is_meta = self.get_kth_bit(state, 28);
        let is_release = self.get_kth_bit(state, 30);
        let is_modifier = is_ctrl || is_alt || is_super || is_hyper || is_meta || is_lock;

        let mut should_reset = false;

        if is_modifier && !is_release {
            // User control like ctrl+v that has nothing to do with us.
            return ModeSwitcherReturn::Done(false);
        }

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
    pub fn get_data_if_continue(&self) -> Option<(Key, bool)> {
        match self {
            ModeSwitcherReturn::Continue(key, should_reset) => {
                Some((key.clone(), should_reset.clone()))
            }
            ModeSwitcherReturn::Done(_) => None,
        }
    }

    pub fn get_data_if_early_return(&self) -> Option<bool> {
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
