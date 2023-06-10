use zbus::dbus_interface;
use zvariant::ObjectPath;

pub struct FcpFactory {

}

pub struct FcpEngine {

}

pub struct FcpService {

}

#[dbus_interface(name = "org.freedesktop.IBus.Factory")]
impl FcpFactory {
    pub fn create_engine(&self, name: &str) -> ObjectPath {
        println!("create_engine called by IBus.");
        ObjectPath::from_str_unchecked("/org/freedesktop/IBus/Engine/FcPinyin")
    }
}

#[dbus_interface(name = "org.freedesktop.IBus.Engine")]
impl FcpEngine {
    pub fn process_key_event(&self, keyval: u32, keycode: u32, state: u32) -> bool {
        println!("process_key_event called by IBus.");
        return false;
    }
}

#[dbus_interface(name = "org.freedesktop.IBus.Service")]
impl FcpService {
    pub fn destroy(&self) {
        println!("destroy called by IBus.");
    }
}