use zbus::{dbus_interface, Connection};
use zvariant::ObjectPath;

use crate::{
    engine::FcpEngine,
    generated::PanelProxy,
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
    engine: FcpEngine,
}

#[dbus_interface(name = "org.freedesktop.IBus.Engine")]
impl InputListener {
    pub async fn process_key_event(&self, keyval: u32, keycode: u32, state: u32) -> bool {
        if state != 0 {
            // If it's not "pressed" state, do nothing.
            return false;
        }
        println!("keyval: {keyval}, keycode: {keycode}, state: {state}");

        return self.engine.on_key_press(keyval).await;
    }
}

pub fn new_input_listener(conn: &Connection) -> InputListener {
    InputListener {
        engine: FcpEngine::new(conn),
    }
}
