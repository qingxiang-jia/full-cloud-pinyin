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

use super::candidate::Candidate;

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
    pub fn into_struct<'a>(self, engine_desc: IBusEngineDesc) -> Structure<'a> {
        let sb: StructureBuilder = StructureBuilder::new();
        let attachments: HashMap<String, Value> = HashMap::new();
        let observed_paths: Vec<Value> = Vec::new();
        let mut engine_list: Vec<Value> = Vec::new();
        let engine_desc_s = engine_desc.into_struct();
        engine_list.push(Value::from(engine_desc_s));
        let s = sb
            .add_field("IBusComponent")
            .add_field(attachments)
            .add_field(self.component_name)
            .add_field(self.description)
            .add_field(self.version)
            .add_field(self.license)
            .add_field(self.author)
            .add_field(self.homepage)
            .add_field(self.exec)
            .add_field(self.textdomain)
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
    pub fn into_struct<'a>(self) -> Structure<'a> {
        let sb: StructureBuilder = StructureBuilder::new();
        let attachments: HashMap<String, Value> = HashMap::new();
        let s = sb
            .add_field("IBusEngineDesc")
            .add_field(attachments)
            .add_field(self.engine_name)
            .add_field(self.long_name)
            .add_field(self.description)
            .add_field(self.language)
            .add_field(self.license)
            .add_field(self.author)
            .add_field(self.icon)
            .add_field(self.layout)
            .add_field(self.rank as u32)
            .add_field(self.hotkeys)
            .add_field(self.symbol)
            .add_field(self.setup)
            .add_field(self.layout_variant)
            .add_field(self.layout_option)
            .add_field(self.version)
            .add_field(self.textdomain)
            .build();
        return s;
    }
}

pub struct IBusLookupTable {
    page_size: u32,
    cursor_pos: u32,
    cursor_visible: bool,
    round: bool,
    orientation: i32,
    candidates: Vec<String>,
    labels: Vec<String>,
}

impl IBusLookupTable {
    pub fn from_candidates(cands: &[Candidate]) -> IBusLookupTable {
        let mut candidates: Vec<String> = Vec::new();
        for cand in cands {
            candidates.push(cand.word.clone());
        }

        IBusLookupTable {
            page_size: 5,
            cursor_pos: 0,
            cursor_visible: false,
            round: false,
            orientation: 2,
            candidates,
            labels: Vec::new(),
        }
    }

    pub fn from_nothing() -> IBusLookupTable {
        IBusLookupTable {
            page_size: 5,
            cursor_pos: 0,
            cursor_visible: false,
            round: false,
            orientation: 2,
            candidates: Vec::new(),
            labels: Vec::new(),
        }
    }

    pub fn into_struct<'a>(self) -> Structure<'a> {
        let sb: StructureBuilder = StructureBuilder::new();

        let attachments: HashMap<String, Value> = HashMap::new();

        let mut cand_texts: Vec<IBusText> = Vec::new();
        for cand in self.candidates {
            cand_texts.push(IBusText { text: cand });
        }
        let mut cands_v: Vec<Value> = Vec::new();
        for cand in cand_texts {
            cands_v.push(Value::from(cand.into_struct()));
        }

        let labels_v: Vec<Value> = Vec::new();
        // According to the goibus implementation, Labels are also IBusText.

        let s = sb
            .add_field("IBusLookupTable")
            .add_field(attachments)
            .add_field(self.page_size)
            .add_field(self.cursor_pos)
            .add_field(self.cursor_visible)
            .add_field(self.round)
            .add_field(self.orientation)
            .add_field(cands_v)
            .add_field(labels_v)
            .build();
        return s;
    }
}

pub struct IBusText {
    pub text: String,
}

impl IBusText {
    pub fn from_str_ref(content: &str) -> IBusText {
        IBusText {
            text: content.clone().to_owned(),
        }
    }

    pub fn into_struct<'a>(self) -> Structure<'a> {
        let sb: StructureBuilder = StructureBuilder::new();
        let attachments: HashMap<String, Value> = HashMap::new();
        let attribute_list = IBusAttrList {};
        let s = sb
            .add_field("IBusText")
            .add_field(attachments)
            .add_field(self.text)
            .add_field(Value::from(attribute_list.into_struct()))
            .build();
        return s;
    }
}

pub struct IBusAttrList {}

impl IBusAttrList {
    pub fn into_struct<'a>(self) -> Structure<'a> {
        let sb = StructureBuilder::new();
        let attachments: HashMap<String, Value> = HashMap::new();
        let attribute_list: Vec<Value> = Vec::new();
        let s = sb
            .add_field("IBusAttrList")
            .add_field(attachments)
            .add_field(attribute_list)
            .build();
        return s;
    }
}

// Currently not used because we can just use a Vec<Value> to represent attribute_list in IBusText.
pub struct IBusAttribute {
    attribute_type: u32,
    value: u32,
    start_index: u32,
    end_index: u32,
}

impl IBusAttribute {
    pub fn into_struct<'a>(self) -> Structure<'a> {
        let sb = StructureBuilder::new();
        let s = sb
            .add_field("IBusAttribute")
            .add_field(self.attribute_type as u32)
            .add_field(self.value as u32)
            .add_field(self.start_index as u32)
            .add_field(self.end_index as u32)
            .build();

        return s;
    }
}
