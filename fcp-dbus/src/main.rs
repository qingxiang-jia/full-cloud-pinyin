#![feature(fmt_helpers_for_derive)]

use std::collections::{HashMap, VecDeque};
use std::fs;

use crate::ibus::dbus_client::ibus_proxy::IBusProxy;
use crate::ibus::dbus_client::panel_proxy::IBusPanelProxy;
use dbus::arg::{RefArg, Variant};
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

    let component = gen_ibus_component();

    match ibus.register_component(component) {
        Ok(()) => println!("Component registration successful!"),
        Err(e) => {
            println!("Failed to register component.");
            display_debus_error(&e);
        },
    }
    
    cr.serve(&conn);
}

fn gen_engine_desc() -> dbus::arg::Variant<Box<dyn RefArg>> {
    let attachments: HashMap<String, Variant<Box<dyn RefArg>>> = HashMap::new();
    
    let mut v: VecDeque<Box<dyn RefArg>> =  VecDeque::new();
    v.push_back(Box::new("IBusEngineDesc".to_owned()));
    v.push_back(Box::new(attachments));
    v.push_back(Box::new("full-cloud-pinyin".to_owned()));
    v.push_back(Box::new("Full Cloud Pinyin".to_owned()));
    v.push_back(Box::new("The Full Cloud Pinyin input method".to_owned()));
    v.push_back(Box::new("zh_cn".to_owned()));
    v.push_back(Box::new("MIT".to_owned()));
    v.push_back(Box::new("Qingxiang Jia".to_owned()));
    v.push_back(Box::new("/usr/share/icons/breeze/emblems/24@3x/emblem-checked.svg".to_owned()));
    v.push_back(Box::new("".to_owned()));
    v.push_back(Box::new(0 as u32));
    v.push_back(Box::new("".to_owned()));
    v.push_back(Box::new("云".to_owned()));
    v.push_back(Box::new("".to_owned()));
    v.push_back(Box::new("".to_owned()));
    v.push_back(Box::new("".to_owned()));
    v.push_back(Box::new("0.1".to_owned()));
    v.push_back(Box::new("full-cloud-pinyin".to_owned()));

    return dbus::arg::Variant(Box::new(v));
}

fn gen_ibus_component() -> dbus::arg::Variant<Box<dyn RefArg>> {
    let attachments: HashMap<String, Variant<Box<dyn RefArg>>> = HashMap::new();
    let observed_paths: Vec<Variant<Box<dyn RefArg>>> = Vec::new();
    let mut engine_list: Vec<Variant<Box<dyn RefArg>>> = Vec::new();
    let engine_desc = gen_engine_desc();
    engine_list.push(engine_desc);

    let mut v: VecDeque<Box<dyn RefArg>> = VecDeque::new();
    v.push_back(Box::new("IBusComponent".to_owned()));
    v.push_back(Box::new(attachments));
    v.push_back(Box::new("FCP Component".to_owned()));
    v.push_back(Box::new("Full Cloud Pinyin".to_owned()));
    v.push_back(Box::new("0.1".to_owned()));
    v.push_back(Box::new("MIT".to_owned()));
    v.push_back(Box::new("Qingxiang Jia".to_owned()));
    v.push_back(Box::new("https://github.com/qingxiang-jia/full-cloud-pinyin/".to_owned()));
    v.push_back(Box::new("".to_owned()));
    v.push_back(Box::new("full-cloud-pinyin".to_owned()));
    v.push_back(Box::new(observed_paths));
    v.push_back(Box::new(engine_list));

    return dbus::arg::Variant(Box::new(v));
}

fn display_debus_error(e: &dbus::Error) {
    let name = e.name().expect("Failed to get name.");
    let message = e.message().expect("Failed to get message");
    println!("DBus error: {name} - {message}");
}

// TODO: the implementation of the factory interface should publish the engine when IBus calls.
