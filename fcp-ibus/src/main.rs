#![feature(fmt_helpers_for_derive)]

use std::fs;

use crate::ibus::dbus_client::ibus_proxy::IBusProxy;
use crate::ibus::dbus_client::panel_proxy::IBusPanelProxy;
use dbus::arg::RefArg;
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
    // Generate IBus client so we can call it.
    let conn = new_ibus_connection();
    let ibus = new_ibus_client(&conn);

    // Verifying the IBus client works.
    let engines = ibus.engines().expect("Failed to get engines.");
    println!("Number of IBus engines: {}", engines.len());

    // Register our input method.
    // let component_xml =
    //     fs::read_to_string("/home/lee/Code/full-cloud-pinyin/fcp-ibus/src/component.xml")
    //         .expect("Unable to read file");
    // let inner = Box::new(component_xml) as Box<dyn RefArg>;
    // let component = dbus::arg::Variant(inner);
    // match ibus.register_component(component) {
    //     Ok(()) => println!("Component registration successful!"),
    //     Err(e) => {
    //         println!("Failed to register component.");
    //         display_debus_error(&e);
    //     },
    // }

    // Generate IBus server so IBus can call us.
    let mut cr = Crossroads::new();
    let engine = FcpEngine {};
    let token: IfaceToken<FcpEngine> = register_org_freedesktop_ibus_engine(&mut cr);
    cr.insert("/", &[token], engine);
    match conn.request_name("org.freedesktop.IBus.FcPinyin", false, true, false) {
        Ok(_) => println!("Request name successful!"),
        Err(e) => {
            println!("Failed to request name.");
            display_debus_error(&e);
        },
    }
    cr.serve(&conn);
}

fn display_debus_error(e: &dbus::Error) {
    let name = e.name().expect("Failed to get name.");
    let message = e.message().expect("Failed to get message");
    println!("DBus error: {name} - {message}");
}

// TODO: the implementation of the factory interface should publish the engine when IBus calls.
