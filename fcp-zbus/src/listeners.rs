use std::{cell::Cell, sync::{Mutex, Arc}};

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

    en_mode_threshold: Arc<Mutex<u8>>,
    en_mode: Arc<Mutex<bool>>,
    
    pub engine: FcpEngine,
}

#[dbus_interface(name = "org.freedesktop.IBus.Engine")]
impl InputListener {
    pub async fn process_key_event(&self, keyval: u32, keycode: u32, state: u32) -> bool {
        // println!("keyval: {keyval}, keycode: {keycode}, state: {state}");

        let maybe_key = Key::from_u32(keyval);
        if maybe_key.is_none() {
            return false; // Not something we want to handle.
        }
        let key = maybe_key.expect("maybe_key is None but shouldn't be.");
 
        if key == Key::Shift {
            if state == 0 {
                self.set_en_mode(false);
            }
            if state == 1073741825 {
                let prev = self.is_en_mode();
                self.set_en_mode(!prev);
            }
        } else {
            self.set_en_mode(false);
        }

        if key != Key::Shift && state == 1073741824 {
            return false; // We don't care about release state.
        }

        if self.is_en_mode() {
            return false;
        }

        return self.engine.on_input(key).await;
    }

    fn is_en_mode(&self) -> bool {
        self.en_mode.lock().expect("Failed to lock en_mode.").clone()
    }

    fn set_en_mode(&self, val: bool) {
        let mut en_mode = self.en_mode.lock().expect("Failed to lock en_mode.");
        *en_mode = val;
    }

    fn get_en_mode_threshold(&self) -> u8 {
        self.en_mode_threshold.lock().expect("Failed to lock en_mode_threshold.").clone()
    }

    fn set_en_mode_threshold(&self, val: u8) {
        let mut threshold = self.en_mode_threshold.lock().expect("Failed to lock en_mode_threshold.");
        *threshold = val;
    }
}

pub fn new_input_listener(conn: &Connection) -> InputListener {
    InputListener {
        en_mode_threshold: Arc::new(Mutex::new(0)),
        en_mode: Arc::new(Mutex::new(false)),
        engine: FcpEngine::new(conn),
    }
}
