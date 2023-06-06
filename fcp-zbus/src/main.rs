#![feature(fmt_helpers_for_derive)]

use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    io::BufRead,
    path::{Path, PathBuf},
};
use zbus::{export::serde, zvariant::Type};

use zbus::{
    blocking::ConnectionBuilder,
    zvariant::{Structure, StructureBuilder, Value},
};

use crate::ibus::proxy_zbus::ibus::IBusProxyBlocking;

mod ibus;

#[derive(Deserialize, Serialize, Type, PartialEq, Debug)]
pub struct Component {
    pub name: String,
    pub description: String,
    pub version: String,
    pub license: String,
    pub author: String,
    pub homepage: String,
    pub command_line: String,
    pub textdomain: String,
}

pub fn test_signature() {
    // Signature of component
    assert_eq!(
        <(
            String,
            HashMap<String, Value>,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            Vec<Value>,
            Vec<Value>
        )>::signature(),
        "(sa{sv}ssssssssavav)"
    );

    // Signature of engine description *with last two strings removed* due to limited support from zvariant
    assert_eq!(
        <(
            String,
            HashMap<String, Value>,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            String,
            u32,
            String,
            String,
            String,
            String,
            String
        )>::signature(),
        "(sa{sv}ssssssssusssss)"
    );

    // Check the signature of a variant of (ii), turns out it's still (ii)
    {
        let v = Value::from((1, 1));
        let sig = v.value_signature();
        assert_eq!(sig.to_string(), "(ii)");
    }

    // Check if long tuple (bigger than 16 items) is supporeted: no, so use StructureBuilder
    {
        let sb = StructureBuilder::new();
        let structure = sb
            .add_field(1)
            .add_field(2)
            .add_field(3)
            .add_field(4)
            .add_field(5)
            .add_field(6)
            .add_field(7)
            .add_field(8)
            .add_field(9)
            .add_field(10)
            .add_field(11)
            .add_field(12)
            .add_field(13)
            .add_field(14)
            .add_field(15)
            .add_field(16)
            .add_field(17)
            .add_field(18)
            .build();
        let sig = structure.signature();
        assert_eq!(sig.to_string(), "(iiiiiiiiiiiiiiiiii)");

        // That Structure can also be converted to a Value aka DBus variant with the signature unchanged
        let v = Value::from(structure);
        assert_eq!(v.value_signature().to_string(), "(iiiiiiiiiiiiiiiiii)");
    }
}

/*
( <- struct
    s <- name
    a{sv} <- attachments: map<string, variant>
    s <- component_name
    s <- description
    s <- version
    s <- license
    s <- author
    s <- homepage
    s <- command_line
    s <- textdomain
    av <- observed_paths
    av <- engline_list ( struct
        s <- name
        a{sv} <- attachments: map<string, variant>
        s <- engine_name
        s <- long_name
        s <- description
        s <- language
        s <- license
        s <- author
        s <- icon
        s <- layout
        u <- rank
        s <- hotkeys
        s <- symbol
        s <- setup
        s <- layout_variant
        s <- layout_option
        s <- version
        s <- textdomain
    )
)
*/

pub fn gen_engine_desc() -> Structure<'static> {
    let sb: StructureBuilder = StructureBuilder::new();
    let attachments: HashMap<String, Value> = HashMap::new();
    let s = sb.add_field("IBusEngineDesc")
        .add_field(attachments)
        .add_field("full-cloud-pinyin")
        .add_field("Full Cloud Pinyin")
        .add_field("The Full Cloud Pinyin input method")
        .add_field("zh_cn")
        .add_field("MIT")
        .add_field("Qingxiang Jia")
        .add_field("/usr/share/icons/breeze/emblems/24@3x/emblem-checked.svg")
        .add_field("")
        .add_field(0 as u32)
        .add_field("")
        .add_field("云")
        .add_field("")
        .add_field("")
        .add_field("")
        .add_field("0.1")
        .add_field("full-cloud-pinyin")
        .build();
    println!("engine desc sig: {}", s.signature().to_string());
    return s;
}

pub fn gen_component() -> Structure<'static> {
    let sb: StructureBuilder = StructureBuilder::new();
    let attachments: HashMap<String, Value> = HashMap::new();
    let observed_paths: Vec<Value> = Vec::new();
    let mut engine_list: Vec<Value> = Vec::new();
    let engine_desc = gen_engine_desc();
    engine_list.push(Value::from(engine_desc));
    let s = sb.add_field("IBusComponent")
        .add_field(attachments)
        .add_field("FCP Component")
        .add_field("Full Cloud Pinyin")
        .add_field("0.1")
        .add_field("MIT")
        .add_field("Qingxiang Jia")
        .add_field("https://github.com/qingxiang-jia/full-cloud-pinyin/")
        .add_field("")
        .add_field("full-cloud-pinyin")
        .add_field(observed_paths)
        .add_field(engine_list)
        .build();
    println!("component sig: {}", s.signature().to_string());
    return s;
}

fn main() {
    let address = get_ibus_address().expect("Failed to get IBus address.");
    println!("address: {address}");

    let conn = ConnectionBuilder::address(address.to_owned().as_str())
        .expect("The address didn't work.")
        .build()
        .expect("Failed to build connection to IBus.");

    let ibus = IBusProxyBlocking::new(&conn).expect("Failed to create IBus proxy.");

    let component = gen_component();

    match ibus.register_component(&Value::from(component)) {
        Ok(_) => println!("Register componnet successfully!"),
        Err(e) => println!("Failed to register component! {e}"),
    }

    test_signature();
}

// Taken from: https://github.com/ArturKovacs/ibus-rs/blob/main/src/lib.rs
pub fn get_ibus_address() -> Result<String, String> {
    if let Ok(addr) = std::env::var("IBUS_ADDRESS") {
        return Ok(addr);
    }

    let display;
    if let Ok(disp) = std::env::var("DISPLAY") {
        display = disp;
    } else {
        display = ":0.0".into();
    }
    let mut split = display.split(":");
    let mut host = split.next().map_or_else(
        || Err(String::from("Failed to get host from display")),
        |x| Ok(x),
    )?;
    let disp_num = split.next().map_or_else(
        || {
            Err(String::from(
                "Failed to get display number from display (colon)",
            ))
        },
        |x| {
            x.split(".").next().map_or_else(
                || Err("Failed to get display number from display (period)".into()),
                |x| Ok(x),
            )
        },
    )?;
    if host.len() == 0 {
        host = "unix";
    }

    let config_home: PathBuf;
    if let Ok(cfg_home) = std::env::var("XDG_CONFIG_HOME") {
        config_home = cfg_home.into();
    } else {
        if let Ok(home) = std::env::var("HOME") {
            config_home = Path::new(&home).join(".config");
        } else {
            return Err("Could not find the home config folder".into());
        }
    }

    let machine_id = get_machine_id()?;
    let mut addr_filename = config_home.clone();
    addr_filename = addr_filename.join("ibus/bus");
    addr_filename = addr_filename.join(format!("{}-{}-{}", machine_id, host, disp_num));

    let addr_file = std::fs::File::open(&addr_filename)
        .map_err(|e| format!("Couldn't open {:?}, err was: {}", addr_filename, e))?;
    let reader = std::io::BufReader::new(addr_file);
    let prefix = "IBUS_ADDRESS=";
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let line = line.trim_start();
                if let Some(addr) = line.strip_prefix(prefix) {
                    return Ok(addr.to_owned());
                }
            }
            Err(e) => {
                return Err(format!(
                    "Failed to read line from the ibus address file: {}",
                    e
                ));
            }
        }
    }
    Err(format!("Failed to find {:?} in the address file", prefix))
}

// Taken from: https://github.com/ArturKovacs/ibus-rs/blob/main/src/lib.rs
fn get_machine_id() -> Result<String, String> {
    if let Ok(id) = std::fs::read_to_string("/etc/machine-id") {
        return Ok(id.trim().to_owned());
    }
    if let Ok(id) = std::fs::read_to_string("/var/lib/dbus/machine-id") {
        return Ok(id.trim().to_owned());
    }
    Err("Could not get the machine id".into())
}
