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

pub struct IBusComponent {
    pub component_name: String,
    pub description: String,
    pub version: String,
    pub license: String,
    pub author: String,
    pub homepage: String,
    pub exec: String,
    pub textdomain: String,
}

impl IBusComponent {
    pub fn into_struct<'a>(&'a self, engine_desc: &'a IBusEngineDesc) -> Structure {
        let sb: StructureBuilder = StructureBuilder::new();
        let attachments: HashMap<String, Value> = HashMap::new();
        let observed_paths: Vec<Value> = Vec::new();
        let mut engine_list: Vec<Value> = Vec::new();
        let engine_desc_s = engine_desc.into_struct();
        engine_list.push(Value::from(engine_desc_s));
        let s = sb
            .add_field("IBusComponent")
            .add_field(attachments)
            .add_field(self.component_name.clone())
            .add_field(self.description.clone())
            .add_field(self.version.clone())
            .add_field(self.license.clone())
            .add_field(self.author.clone())
            .add_field(self.homepage.clone())
            .add_field(self.exec.clone())
            .add_field(self.textdomain.clone())
            .add_field(observed_paths)
            .add_field(engine_list)
            .build();
        return s;
    }
}

pub struct IBusEngineDesc {
    pub engine_name: String,
    pub long_name: String,
    pub description: String,
    pub language: String,
    pub license: String,
    pub author: String,
    pub icon: String,
    pub layout: String,
    pub rank: u32,
    pub hotkeys: String,
    pub symbol: String,
    pub setup: String,
    pub layout_variant: String,
    pub layout_option: String,
    pub version: String,
    pub textdomain: String,
}

impl IBusEngineDesc {
    pub fn into_struct(&self) -> Structure {
        let sb: StructureBuilder = StructureBuilder::new();
        let attachments: HashMap<String, Value> = HashMap::new();
        let s = sb
            .add_field("IBusEngineDesc")
            .add_field(attachments)
            .add_field(self.engine_name.clone())
            .add_field(self.long_name.clone())
            .add_field(self.description.clone())
            .add_field(self.language.clone())
            .add_field(self.license.clone())
            .add_field(self.author.clone())
            .add_field(self.icon.clone())
            .add_field(self.layout.clone())
            .add_field(self.rank as u32)
            .add_field(self.hotkeys.clone())
            .add_field(self.symbol.clone())
            .add_field(self.setup.clone())
            .add_field(self.layout_variant.clone())
            .add_field(self.layout_option.clone())
            .add_field(self.version.clone())
            .add_field(self.textdomain.clone())
            .build();
        return s;
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
