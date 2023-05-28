use std::time::Duration;

use dbus::blocking::{Connection, Proxy};

pub fn new_ibus_client(conn: &Connection) -> Proxy<&Connection> {
    conn.with_proxy(
        "org.freedesktop.IBus",
        "/org/freedesktop/IBus",
        Duration::from_millis(1000),
    )
}
