use reqwest;
use zbus::{dbus_interface, Connection};
use zvariant::ObjectPath;

use crate::generated::{IBusProxy, PanelProxy};

pub struct FcpFactory {}

#[dbus_interface(name = "org.freedesktop.IBus.Factory")]
impl FcpFactory {
    pub fn create_engine(&self, name: &str) -> ObjectPath {
        println!("create_engine called by IBus.");
        ObjectPath::from_str_unchecked("/org/freedesktop/IBus/Engine/FcPinyin")
    }
}

pub struct FcpService {}

#[dbus_interface(name = "org.freedesktop.IBus.Service")]
impl FcpService {
    pub fn destroy(&self) {
        println!("destroy called by IBus.");
    }
}

pub struct FcpEngine<'a> {
    ibus: IBusProxy<'a>,
    panel: PanelProxy<'a>,
    http: reqwest::Client,
}

#[dbus_interface(name = "org.freedesktop.IBus.Engine")]
impl FcpEngine<'static> {
    pub async fn process_key_event(&self, keyval: u32, keycode: u32, state: u32) -> bool {
        println!("process_key_event called by IBus.");
        return false;
    }
}

pub async fn new_fcp_engine(conn: Connection) -> FcpEngine<'static> {
    let ibus = IBusProxy::new(&conn)
        .await
        .expect("Failed to create IBusProxy.");

    let panel = PanelProxy::new(&conn)
        .await
        .expect("Failed to create PanelProxy.");

    FcpEngine {
        ibus,
        panel,
        http: reqwest::Client::new(),
    }
}
