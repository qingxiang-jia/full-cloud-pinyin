use zbus::dbus_interface;
use zvariant::ObjectPath;

use crate::{
    engine::FcpEngine,
    keys::{Key, KeyVal},
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

        // Pressed = 0, Released = 1073741825
        let shift_pressed = if state == 0 { true } else { false };

        let key_sym_val = KeyVal::from_u32(keyval);
        if key_sym_val.is_none() {
            return false; // Not something we want to handle.
        }
        let key = Key::from_key_val(
            key_sym_val.expect("key_sym_val can't be None but is None."),
            shift_pressed,
        );

        return self.engine.on_input(key).await;
    }
}
