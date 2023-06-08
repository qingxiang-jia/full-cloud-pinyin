use zvariant::ObjectPath;

pub struct Fcp {

}

impl Fcp {
    pub fn create_engine(&self, name: &str) -> ObjectPath {
        ObjectPath::from_str_unchecked("/org/freedesktop/IBus/Engine/FcPinyin")
    }
}