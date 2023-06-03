#![feature(fmt_helpers_for_derive)]

use std::{
    io::BufRead,
    path::{Path, PathBuf},
};

use zbus::{blocking::ConnectionBuilder, zvariant::{Value, Structure, StructureBuilder}};

use crate::ibus::proxy_zbus::ibus::IBusProxyBlocking;

mod ibus;

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

/*
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


 */

fn main() {
    let address = get_ibus_address().expect("Failed to get IBus address.");
    println!("address: {address}");

    let conn = ConnectionBuilder::address(address.to_owned().as_str())
        .expect("The address didn't work.")
        .build()
        .expect("Failed to build connection to IBus.");

    let ibus = IBusProxyBlocking::new(&conn).expect("Failed to create IBus proxy.");
    
    let component = Component {
        name: "org.freedesktop.IBus.Fcpinyin".to_owned(),
        description: "Full Cloud Pinyin".to_owned(),
        version: "0.1".to_owned(),
        license: "MIT".to_owned(),
        author: "Qingxiang Jia".to_owned(),
        homepage: "https://github.com/qingxiang-jia/full-cloud-pinyin/".to_owned(),
        command_line: "".to_owned(),
        textdomain: "full-cloud-pinyin".to_owned(),
    };

    let v = Value::from(("org.freedesktop.IBus.Fcpinyin".to_owned(), "Full Cloud Pinyin".to_owned(), "0.1".to_owned(), "MIT".to_owned(), "Qingxiang Jia".to_owned(), "https://github.com/qingxiang-jia/full-cloud-pinyin/".to_owned(), "".to_owned(), "full-cloud-pinyin".to_owned()));

    match ibus.register_component(&v) {
        Ok(_) => println!("Register componnet successfully!"),
        Err(e) => println!("Failed to register component! {e}"),
    }
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
