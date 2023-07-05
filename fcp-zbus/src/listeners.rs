use zbus::dbus_interface;
use zvariant::ObjectPath;

use crate::{
    engine::FcpEngine,
    keys::Key,
};

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
    pub engine: FcpEngine,
}

#[dbus_interface(name = "org.freedesktop.IBus.Engine")]
impl InputListener {
    pub async fn process_key_event(&self, keyval: u32, keycode: u32, state: u32) -> bool {
        println!("keyval: {keyval}, keycode: {keycode}, state: {state}");

        if state != 0 {
            // When state not equal to 0, it's not a pressed action, could be releasing of
            // the key which we don't care.
            return false;
        }

        let key = Key::from_u32(keyval);
        if key.is_none() {
            return false; // Not something we want to handle.
        }

        return self.engine.on_input(key.expect("key is None but shouldn't be.")).await;
    }
}
