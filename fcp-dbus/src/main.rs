#![feature(fmt_helpers_for_derive)]

use std::collections::{HashMap, VecDeque};
use std::fs;

use crate::ibus::dbus_client::ibus_proxy::IBusProxy;
use crate::ibus::dbus_client::manual::{Component, EngineDesc};
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

    let component = Component {
        name: "org.freedesktop.IBus.Fcpinyin".to_owned(),
        description: "Full Cloud Pinyin".to_owned(),
        version: "0.1".to_owned(),
        license: "MIT".to_owned(),
        author: "Qingxiang Jia".to_owned(),
        homepage: "https://github.com/qingxiang-jia/full-cloud-pinyin/".to_owned(),
        exec: "".to_owned(),
        textdomain: "full-cloud-pinyin".to_owned(),
        attachments: HashMap::new(),
        engines: [EngineDesc {
            attachments: HashMap::new(),
            name: "full-cloud-pinyin".to_owned(),
            longname: "Full Cloud Pinyin".to_owned(),
            description: "Full Cloud Pinyin".to_owned(),
            language: "en".to_owned(),
            license: "MIT".to_owned(),
            author: "Qingxiang Jia".to_owned(),
            icon: "/usr/share/icons/breeze/emblems/24@3x/emblem-checked.svg".to_owned(),
            layout: "us".to_owned(),
            rank: 0,
            hotkeys: "".to_owned(),
            symbol: "".to_owned(),
            setup: "".to_owned(),
            layout_option: "".to_owned(),
            layout_variant: "".to_owned(),
            version: "0.1".to_owned(),
            text_domain: "full-cloud-pinyin".to_owned(),
        }]
    };

    let tuple = (component.name, component.description, component.version, component.license, component.author, component.homepage, component.exec, component.textdomain, component.attachments);

    let componnet_variant = Box::new(tuple) as Box<dyn RefArg>;

    match ibus.register_component(dbus::arg::Variant(componnet_variant)) {
        Ok(()) => println!("Component registration successful!"),
        Err(e) => {
            println!("Failed to register component.");
            display_debus_error(&e);
        },
    }
    
    cr.serve(&conn);
}

fn get_engine_desc_tuple() -> Box<dyn RefArg> {
    let attachments: HashMap<String, Box<dyn RefArg>> = HashMap::new();
    let mut v: VecDeque<Box<dyn RefArg>> =  VecDeque::new();
    v.push_back(Box::new("org.freedesktop.IBus.Fcpinyin".to_owned()));
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
    
    return Box::new(v as Box<dyn RefArg>);
}

fn display_debus_error(e: &dbus::Error) {
    let name = e.name().expect("Failed to get name.");
    let message = e.message().expect("Failed to get message");
    println!("DBus error: {name} - {message}");
}

// TODO: the implementation of the factory interface should publish the engine when IBus calls.
