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
    // No one will modify it concurrently, it's just the Rust
    // compiler isn't able to prove that.
    en_mode: Arc<Mutex<bool>>,
    pub engine: FcpEngine,
}

#[dbus_interface(name = "org.freedesktop.IBus.Engine")]
impl InputListener {
    pub async fn process_key_event(&self, keyval: u32, keycode: u32, state: u32) -> bool {
        // let bi = format!("{state:b}");
        // println!("keyval: {keyval}, keycode: {keycode}, state: {bi}");

        // State flags
        let is_release = self.get_kth_bit(state, 30);
        let is_ctrl = self.get_kth_bit(state, 2);

        if is_ctrl && is_release {
            let was_en_mode = self.is_en_mode();
            self.set_en_mode(!was_en_mode);
        }
        if is_ctrl && !is_release {
            // User control like ctrl+v that has nothing to do with us.
            return false;
        }

        if self.is_en_mode() || is_release {
            return false;
        }

        let maybe_key = Key::from_u32(keyval);
        if maybe_key.is_none() {
            return false; // We don't handle anything outside of key.
        }
        let key = maybe_key.expect("maybe_key is None but it shouldn't.");

        return self.engine.on_input(key).await;
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

    fn get_kth_bit(&self, n: u32, k: u32) -> bool {
        (n & (1 << k)) >> k == 1
    }
}

pub fn new_input_listener(conn: &Connection) -> InputListener {
    InputListener {
        en_mode: Arc::new(Mutex::new(false)),
        engine: FcpEngine::new(conn),
    }
}
