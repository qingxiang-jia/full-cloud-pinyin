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

use std::collections::HashMap;

use zvariant::{Structure, StructureBuilder, Value};

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