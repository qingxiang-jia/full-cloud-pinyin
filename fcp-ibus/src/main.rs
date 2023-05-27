#![feature(fmt_helpers_for_derive)]

use ibus::{client_factory::new_ibus_client, dbus_server::engine::{register_org_freedesktop_ibus_engine, IBusEngine}, fcp_engine::FcpEngine};
use dbus_crossroads::{self, Crossroads, IfaceToken};
use ibus::fcp_engine;
mod ibus;

fn main() {
    let ibus = new_ibus_client();
    let mut cr = Crossroads::new();
    let engine = FcpEngine {};
    let token: IfaceToken<FcpEngine> = register_org_freedesktop_ibus_engine(&mut cr);
    cr.insert("/", &[token], engine);
}
