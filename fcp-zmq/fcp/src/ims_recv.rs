// Automatically generated rust module for 'ims_recv.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageInfo, MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CommandToFcitx<'a> {
    pub command: mod_CommandToFcitx::OneOfcommand<'a>,
}

impl<'a> MessageRead<'a> for CommandToFcitx<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.command = mod_CommandToFcitx::OneOfcommand::commit_text(r.read_message::<CommitText>(bytes)?),
                Ok(18) => msg.command = mod_CommandToFcitx::OneOfcommand::update_preedut(r.read_message::<UpdatePreedit>(bytes)?),
                Ok(26) => msg.command = mod_CommandToFcitx::OneOfcommand::update_lt(r.read_message::<UpdateLookuptable>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CommandToFcitx<'a> {
    fn get_size(&self) -> usize {
        0
        + match self.command {
            mod_CommandToFcitx::OneOfcommand::commit_text(ref m) => 1 + sizeof_len((m).get_size()),
            mod_CommandToFcitx::OneOfcommand::update_preedut(ref m) => 1 + sizeof_len((m).get_size()),
            mod_CommandToFcitx::OneOfcommand::update_lt(ref m) => 1 + sizeof_len((m).get_size()),
            mod_CommandToFcitx::OneOfcommand::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.command {            mod_CommandToFcitx::OneOfcommand::commit_text(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_CommandToFcitx::OneOfcommand::update_preedut(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_CommandToFcitx::OneOfcommand::update_lt(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_CommandToFcitx::OneOfcommand::None => {},
    }        Ok(())
    }
}

pub mod mod_CommandToFcitx {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfcommand<'a> {
    commit_text(CommitText<'a>),
    update_preedut(UpdatePreedit<'a>),
    update_lt(UpdateLookuptable<'a>),
    None,
}

impl<'a> Default for OneOfcommand<'a> {
    fn default() -> Self {
        OneOfcommand::None
    }
}

}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct CommitText<'a> {
    pub text: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for CommitText<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.text = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for CommitText<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.text == "" { 0 } else { 1 + sizeof_len((&self.text).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.text != "" { w.write_with_tag(10, |w| w.write_string(&**&self.text))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdatePreedit<'a> {
    pub text: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for UpdatePreedit<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.text = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UpdatePreedit<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.text == "" { 0 } else { 1 + sizeof_len((&self.text).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.text != "" { w.write_with_tag(10, |w| w.write_string(&**&self.text))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct UpdateLookuptable<'a> {
    pub lt: Option<LookupTable<'a>>,
}

impl<'a> MessageRead<'a> for UpdateLookuptable<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.lt = Some(r.read_message::<LookupTable>(bytes)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UpdateLookuptable<'a> {
    fn get_size(&self) -> usize {
        0
        + self.lt.as_ref().map_or(0, |m| 1 + sizeof_len((m).get_size()))
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if let Some(ref s) = self.lt { w.write_with_tag(10, |w| w.write_message(s))?; }
        Ok(())
    }
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct LookupTable<'a> {
    pub candidates: Vec<Cow<'a, str>>,
}

impl<'a> MessageRead<'a> for LookupTable<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.candidates.push(r.read_string(bytes).map(Cow::Borrowed)?),
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for LookupTable<'a> {
    fn get_size(&self) -> usize {
        0
        + self.candidates.iter().map(|s| 1 + sizeof_len((s).len())).sum::<usize>()
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        for s in &self.candidates { w.write_with_tag(10, |w| w.write_string(&**s))?; }
        Ok(())
    }
}

