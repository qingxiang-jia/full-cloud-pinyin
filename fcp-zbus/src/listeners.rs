use zbus::{dbus_interface, Connection};
use zvariant::ObjectPath;

use crate::pipeline::Pipeline;

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
    pipeline: Pipeline
}

#[dbus_interface(name = "org.freedesktop.IBus.Engine")]
impl InputListener {
    pub async fn process_key_event(&self, keyval: u32, keycode: u32, state: u32) -> bool {
        // let bi = format!("{state:b}");
        // println!("keyval: {keyval}, keycode: {keycode}, state: {bi}");
        self.pipeline.accept(keyval, keycode, state).await
    }
}

pub fn new_input_listener(conn: &Connection) -> InputListener {
    InputListener {
        pipeline: Pipeline::new(conn)
    }
}
