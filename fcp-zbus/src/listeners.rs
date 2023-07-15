use std::sync::{Arc, Mutex};

use zbus::{dbus_interface, Connection};
use zvariant::ObjectPath;

use crate::{engine::FcpEngine, keys::Key};

// We have three interfaces to implement in order to get a working engine, but only the
// org.freedesktop.IBus.Engine matters in practice.

// Implementation of org.freedesktop.IBus.Factory interface

pub struct FactoryListener {}

#[dbus_interface(name = "org.freedesktop.IBus.Factory")]
impl FactoryListener {
    pub fn create_engine(&self, name: &str) -> ObjectPath {
        println!("create_engine called by IBus.");
        ObjectPath::from_str_unchecked("/org/freedesktop/IBus/Engine/FcPinyin")
    }
}

// Implementation of org.freedesktop.IBus.Service interface

pub struct ServiceListener {}

#[dbus_interface(name = "org.freedesktop.IBus.Service")]
impl ServiceListener {
    pub fn destroy(&self) {
        println!("destroy called by IBus.");
    }
}

pub struct InputListener {
    // For these two fields, no one will modify it concurrently, it's just the Rust
    // compiler isn't able to prove that.
    shift_on: Arc<Mutex<bool>>,
    shift_on_and_type: Arc<Mutex<bool>>,
    en_mode: Arc<Mutex<bool>>,

    pub engine: FcpEngine,
}

#[dbus_interface(name = "org.freedesktop.IBus.Engine")]
impl InputListener {
    pub async fn process_key_event(&self, keyval: u32, keycode: u32, state: u32) -> bool {
        // let bi = format!("{state:b}");
        // println!("keyval: {keyval}, keycode: {keycode}, state: {bi}");

        // State flags
        let type_normally = state == 0;
        let type_while_shift_on = self.get_kth_bit(state, 0);
        let is_release = self.get_kth_bit(state, 30);

        let maybe_key = Key::from_u32(keyval);
        if maybe_key.is_none() {
            return false; // We don't handle anything outside of key.
        }
        let key = maybe_key.expect("maybe_key is None but it shouldn't.");

        if type_normally {
            if let Key::Shift = key {
                self.set_shift_on(true);
            } else {
                if self.is_en_mode() {
                    return false;
                } else {
                    return self.engine.on_input(key).await;
                }
            }
        }

        if type_while_shift_on {
            if self.is_en_mode() {
                return false;
            } else {
                self.set_shift_on_and_type(true);
                return self.engine.on_input(key).await;
            }
        }

        if is_release {
            if self.is_shift_on_and_type() {
                self.set_shift_on_and_type(false);
                self.set_shift_on(false);
            } else {
                self.set_shift_on(false);
                if key == Key::Shift {
                    self.set_en_mode(true);
                }
            }
        }
        
        return false;
    }

    fn is_en_mode(&self) -> bool {
        self.en_mode
            .lock()
            .expect("Failed to lock en_mode.")
            .clone()
    }

    fn set_en_mode(&self, val: bool) {
        let mut en_mode = self.en_mode.lock().expect("Failed to lock en_mode.");
        *en_mode = val;
    }

    fn is_shift_on(&self) -> bool {
        self.shift_on
            .lock()
            .expect("Failed to lock shift_on.")
            .clone()
    }

    fn set_shift_on(&self, val: bool) {
        let mut shift_on = self.shift_on.lock().expect("Failed to lock shift_on.");
        *shift_on = val;
    }

    fn is_shift_on_and_type(&self) -> bool {
        self.shift_on_and_type
            .lock()
            .expect("Failed to lock shift_on_then_input.")
            .clone()
    }

    fn set_shift_on_and_type(&self, val: bool) {
        let mut shift_on_then_input = self
            .shift_on_and_type
            .lock()
            .expect("Failed to lock shift_on_then_input.");
        *shift_on_then_input = val;
    }

    fn get_kth_bit(&self, n: u32, k: u32) -> bool {
        (n & (1 << k)) >> k == 1
    }
}

pub fn new_input_listener(conn: &Connection) -> InputListener {
    InputListener {
        shift_on: Arc::new(Mutex::new(false)),
        shift_on_and_type: Arc::new(Mutex::new(false)),
        en_mode: Arc::new(Mutex::new(false)),
        engine: FcpEngine::new(conn),
    }
}
