use zbus::dbus_interface;
use zvariant::ObjectPath;

pub struct Fcp {

}

#[dbus_interface(name = "org.freedesktop.IBus.FcPinyin")]
impl Fcp {
    pub fn create_engine(&self, name: &str) -> ObjectPath {
        println!("create_engine called by IBus.");
        ObjectPath::from_str_unchecked("/org/freedesktop/IBus/Engine/FcPinyin")
    }

    pub fn process_key_event(&self, keyval: u32, keycode: u32, state: u32) -> bool {
        println!("process_key_event called by IBus.");
        return true
    }
}