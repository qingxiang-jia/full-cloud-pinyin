use std::collections::HashMap;

use dbus::arg;

pub struct Component {
    pub name: String,
    pub attachments: HashMap<String, arg::Variant<Box<dyn arg::RefArg>>>,
    pub description: String,
    pub version: String,
    pub license: String,
    pub author: String,
    pub homepage: String,
    pub exec: String,
    pub textdomain: String,
    pub engines: [EngineDesc; 1],
}

pub struct EngineDesc {
    pub attachments: HashMap<String, arg::Variant<Box<dyn arg::RefArg>>>,
    pub name: String,
    pub longname: String,
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
    pub text_domain: String,
}