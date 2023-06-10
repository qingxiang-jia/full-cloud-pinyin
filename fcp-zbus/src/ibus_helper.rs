use std::{
    collections::HashMap,
    io::BufRead,
    path::{Path, PathBuf},
};

use zvariant::{Structure, StructureBuilder, Value};

/*
IBusComponent is a DBus variant. Its actual definition is the following:

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
pub fn gen_ibus_component() -> Structure<'static> {
    let sb: StructureBuilder = StructureBuilder::new();
    let attachments: HashMap<String, Value> = HashMap::new();
    let observed_paths: Vec<Value> = Vec::new();
    let mut engine_list: Vec<Value> = Vec::new();
    let engine_desc = gen_engine_desc();
    engine_list.push(Value::from(engine_desc));
    let s = sb
        .add_field("IBusComponent")
        .add_field(attachments)
        .add_field("org.freedesktop.IBus.FcPinyin")
        .add_field("Full Cloud Pinyin")
        .add_field("0.1")
        .add_field("MIT")
        .add_field("Qingxiang Jia")
        .add_field("https://github.com/qingxiang-jia/full-cloud-pinyin/")
        .add_field("/home/lee/Code/Projects/full-cloud-pinyin/")
        .add_field("full-cloud-pinyin")
        .add_field(observed_paths)
        .add_field(engine_list)
        .build();
    println!("component sig: {}", s.signature().to_string());
    return s;
}

fn gen_engine_desc() -> Structure<'static> {
    let sb: StructureBuilder = StructureBuilder::new();
    let attachments: HashMap<String, Value> = HashMap::new();
    let s = sb
        .add_field("IBusEngineDesc")
        .add_field(attachments)
        .add_field("full-cloud-pinyin")
        .add_field("Full Cloud Pinyin")
        .add_field("The Full Cloud Pinyin input method")
        .add_field("en")
        .add_field("MIT")
        .add_field("Qingxiang Jia")
        .add_field("/usr/share/icons/breeze/emblems/24@3x/emblem-checked.svg")
        .add_field("en")
        .add_field(0 as u32)
        .add_field("/usr/bin/gittupref")
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
