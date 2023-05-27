#![feature(fmt_helpers_for_derive)]

use crate::ibus::dbus_client::ibus_proxy::IBusProxy;
use crate::ibus::dbus_client::panel_proxy::IBusPanelProxy;
use dbus_crossroads::{self, Crossroads, IfaceToken};
use ibus::fcp_engine;
use ibus::{
    client_factory::new_ibus_client,
    connection_factory::new_ibus_connection,
    dbus_server::engine::{register_org_freedesktop_ibus_engine, IBusEngine},
    fcp_engine::FcpEngine,
};
mod ibus;

fn main() {
    let conn = new_ibus_connection();
    let ibus = new_ibus_client(&conn);
    let engines = ibus.engines().expect("Failed to get engines.");
    println!("Number of IBus engines: {}", engines.len());

    let mut cr = Crossroads::new();
    let engine = FcpEngine {};
    let token: IfaceToken<FcpEngine> = register_org_freedesktop_ibus_engine(&mut cr);
    cr.insert("/", &[token], engine);
    cr.serve(&conn);
}

// TODO: the implementation of the factory interface should publish the engine when IBus calls.