//! # DBus interface proxies for: `org.freedesktop.IBus.Service`, `org.freedesktop.IBus.Factory`, `org.freedesktop.IBus.Service`, `org.freedesktop.IBus.Engine`, `org.freedesktop.IBus.Panel`
//!
//! This code was generated by `zbus-xmlgen` `3.1.0` from DBus introspection data.
//! Source: `interfaces.xml`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!

use zbus::dbus_proxy;

#[dbus_proxy(interface = "org.freedesktop.IBus.Factory", assume_defaults = true)]
trait Factory {
    /// CreateEngine method
    fn create_engine(&self, name: &str) -> zbus::Result<zbus::zvariant::OwnedObjectPath>;
}