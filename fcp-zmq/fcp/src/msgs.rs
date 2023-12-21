// Automatically generated rust module for 'msgs.proto' file

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

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KeyEvent {
    NUM_0 = 0,
    NUM_1 = 1,
    NUM_2 = 2,
    NUM_3 = 3,
    NUM_4 = 4,
    NUM_5 = 5,
    NUM_6 = 6,
    NUM_7 = 7,
    NUM_8 = 8,
    NUM_9 = 9,
    A_LWR = 100,
    B_LWR = 101,
    C_LWR = 102,
    D_LWR = 103,
    E_LWR = 104,
    F_LWR = 105,
    G_LWR = 106,
    H_LWR = 107,
    I_LWR = 108,
    J_LWR = 109,
    K_LWR = 110,
    L_LWR = 111,
    M_LWR = 112,
    N_LWR = 113,
    O_LWR = 114,
    P_LWR = 115,
    Q_LWR = 116,
    R_LWR = 117,
    S_LWR = 118,
    T_LWR = 119,
    U_LWR = 120,
    V_LWR = 121,
    W_LWR = 122,
    X_LWR = 123,
    Y_LWR = 124,
    Z_LWR = 125,
    A_UPR = 200,
    B_UPR = 201,
    C_UPR = 202,
    D_UPR = 203,
    E_UPR = 204,
    F_UPR = 205,
    G_UPR = 206,
    H_UPR = 207,
    I_UPR = 208,
    J_UPR = 209,
    K_UPR = 210,
    L_UPR = 211,
    M_UPR = 212,
    N_UPR = 213,
    O_UPR = 214,
    P_UPR = 215,
    Q_UPR = 216,
    R_UPR = 217,
    S_UPR = 218,
    T_UPR = 219,
    U_UPR = 220,
    V_UPR = 221,
    W_UPR = 222,
    X_UPR = 223,
    Y_UPR = 224,
    Z_UPR = 225,
    COMMA = 10,
    PERIOD = 11,
    QEST_MARK = 12,
    EXCL_MARK = 13,
    SEMI_COLON = 14,
    DBL_QUOTE = 15,
    SGL_QUOTE = 16,
    BRKT_OPEN = 17,
    BRKT_CLOSE = 18,
    SLASH = 19,
    BACKSLASH = 20,
    ELLIPSIS = 21,
    ENTER = 30,
    SPACE = 31,
    MINUS = 32,
    EQUAL = 33,
    UP = 40,
    DOWN = 41,
    LEFT = 42,
    RIGHT = 43,
    SHIFT = 50,
    CTRL = 51,
    ALT = 52,
}

impl Default for KeyEvent {
    fn default() -> Self {
        KeyEvent::NUM_0
    }
}

impl From<i32> for KeyEvent {
    fn from(i: i32) -> Self {
        match i {
            0 => KeyEvent::NUM_0,
            1 => KeyEvent::NUM_1,
            2 => KeyEvent::NUM_2,
            3 => KeyEvent::NUM_3,
            4 => KeyEvent::NUM_4,
            5 => KeyEvent::NUM_5,
            6 => KeyEvent::NUM_6,
            7 => KeyEvent::NUM_7,
            8 => KeyEvent::NUM_8,
            9 => KeyEvent::NUM_9,
            100 => KeyEvent::A_LWR,
            101 => KeyEvent::B_LWR,
            102 => KeyEvent::C_LWR,
            103 => KeyEvent::D_LWR,
            104 => KeyEvent::E_LWR,
            105 => KeyEvent::F_LWR,
            106 => KeyEvent::G_LWR,
            107 => KeyEvent::H_LWR,
            108 => KeyEvent::I_LWR,
            109 => KeyEvent::J_LWR,
            110 => KeyEvent::K_LWR,
            111 => KeyEvent::L_LWR,
            112 => KeyEvent::M_LWR,
            113 => KeyEvent::N_LWR,
            114 => KeyEvent::O_LWR,
            115 => KeyEvent::P_LWR,
            116 => KeyEvent::Q_LWR,
            117 => KeyEvent::R_LWR,
            118 => KeyEvent::S_LWR,
            119 => KeyEvent::T_LWR,
            120 => KeyEvent::U_LWR,
            121 => KeyEvent::V_LWR,
            122 => KeyEvent::W_LWR,
            123 => KeyEvent::X_LWR,
            124 => KeyEvent::Y_LWR,
            125 => KeyEvent::Z_LWR,
            200 => KeyEvent::A_UPR,
            201 => KeyEvent::B_UPR,
            202 => KeyEvent::C_UPR,
            203 => KeyEvent::D_UPR,
            204 => KeyEvent::E_UPR,
            205 => KeyEvent::F_UPR,
            206 => KeyEvent::G_UPR,
            207 => KeyEvent::H_UPR,
            208 => KeyEvent::I_UPR,
            209 => KeyEvent::J_UPR,
            210 => KeyEvent::K_UPR,
            211 => KeyEvent::L_UPR,
            212 => KeyEvent::M_UPR,
            213 => KeyEvent::N_UPR,
            214 => KeyEvent::O_UPR,
            215 => KeyEvent::P_UPR,
            216 => KeyEvent::Q_UPR,
            217 => KeyEvent::R_UPR,
            218 => KeyEvent::S_UPR,
            219 => KeyEvent::T_UPR,
            220 => KeyEvent::U_UPR,
            221 => KeyEvent::V_UPR,
            222 => KeyEvent::W_UPR,
            223 => KeyEvent::X_UPR,
            224 => KeyEvent::Y_UPR,
            225 => KeyEvent::Z_UPR,
            10 => KeyEvent::COMMA,
            11 => KeyEvent::PERIOD,
            12 => KeyEvent::QEST_MARK,
            13 => KeyEvent::EXCL_MARK,
            14 => KeyEvent::SEMI_COLON,
            15 => KeyEvent::DBL_QUOTE,
            16 => KeyEvent::SGL_QUOTE,
            17 => KeyEvent::BRKT_OPEN,
            18 => KeyEvent::BRKT_CLOSE,
            19 => KeyEvent::SLASH,
            20 => KeyEvent::BACKSLASH,
            21 => KeyEvent::ELLIPSIS,
            30 => KeyEvent::ENTER,
            31 => KeyEvent::SPACE,
            32 => KeyEvent::MINUS,
            33 => KeyEvent::EQUAL,
            40 => KeyEvent::UP,
            41 => KeyEvent::DOWN,
            42 => KeyEvent::LEFT,
            43 => KeyEvent::RIGHT,
            50 => KeyEvent::SHIFT,
            51 => KeyEvent::CTRL,
            52 => KeyEvent::ALT,
            _ => Self::default(),
        }
    }
}

impl<'a> From<&'a str> for KeyEvent {
    fn from(s: &'a str) -> Self {
        match s {
            "NUM_0" => KeyEvent::NUM_0,
            "NUM_1" => KeyEvent::NUM_1,
            "NUM_2" => KeyEvent::NUM_2,
            "NUM_3" => KeyEvent::NUM_3,
            "NUM_4" => KeyEvent::NUM_4,
            "NUM_5" => KeyEvent::NUM_5,
            "NUM_6" => KeyEvent::NUM_6,
            "NUM_7" => KeyEvent::NUM_7,
            "NUM_8" => KeyEvent::NUM_8,
            "NUM_9" => KeyEvent::NUM_9,
            "A_LWR" => KeyEvent::A_LWR,
            "B_LWR" => KeyEvent::B_LWR,
            "C_LWR" => KeyEvent::C_LWR,
            "D_LWR" => KeyEvent::D_LWR,
            "E_LWR" => KeyEvent::E_LWR,
            "F_LWR" => KeyEvent::F_LWR,
            "G_LWR" => KeyEvent::G_LWR,
            "H_LWR" => KeyEvent::H_LWR,
            "I_LWR" => KeyEvent::I_LWR,
            "J_LWR" => KeyEvent::J_LWR,
            "K_LWR" => KeyEvent::K_LWR,
            "L_LWR" => KeyEvent::L_LWR,
            "M_LWR" => KeyEvent::M_LWR,
            "N_LWR" => KeyEvent::N_LWR,
            "O_LWR" => KeyEvent::O_LWR,
            "P_LWR" => KeyEvent::P_LWR,
            "Q_LWR" => KeyEvent::Q_LWR,
            "R_LWR" => KeyEvent::R_LWR,
            "S_LWR" => KeyEvent::S_LWR,
            "T_LWR" => KeyEvent::T_LWR,
            "U_LWR" => KeyEvent::U_LWR,
            "V_LWR" => KeyEvent::V_LWR,
            "W_LWR" => KeyEvent::W_LWR,
            "X_LWR" => KeyEvent::X_LWR,
            "Y_LWR" => KeyEvent::Y_LWR,
            "Z_LWR" => KeyEvent::Z_LWR,
            "A_UPR" => KeyEvent::A_UPR,
            "B_UPR" => KeyEvent::B_UPR,
            "C_UPR" => KeyEvent::C_UPR,
            "D_UPR" => KeyEvent::D_UPR,
            "E_UPR" => KeyEvent::E_UPR,
            "F_UPR" => KeyEvent::F_UPR,
            "G_UPR" => KeyEvent::G_UPR,
            "H_UPR" => KeyEvent::H_UPR,
            "I_UPR" => KeyEvent::I_UPR,
            "J_UPR" => KeyEvent::J_UPR,
            "K_UPR" => KeyEvent::K_UPR,
            "L_UPR" => KeyEvent::L_UPR,
            "M_UPR" => KeyEvent::M_UPR,
            "N_UPR" => KeyEvent::N_UPR,
            "O_UPR" => KeyEvent::O_UPR,
            "P_UPR" => KeyEvent::P_UPR,
            "Q_UPR" => KeyEvent::Q_UPR,
            "R_UPR" => KeyEvent::R_UPR,
            "S_UPR" => KeyEvent::S_UPR,
            "T_UPR" => KeyEvent::T_UPR,
            "U_UPR" => KeyEvent::U_UPR,
            "V_UPR" => KeyEvent::V_UPR,
            "W_UPR" => KeyEvent::W_UPR,
            "X_UPR" => KeyEvent::X_UPR,
            "Y_UPR" => KeyEvent::Y_UPR,
            "Z_UPR" => KeyEvent::Z_UPR,
            "COMMA" => KeyEvent::COMMA,
            "PERIOD" => KeyEvent::PERIOD,
            "QEST_MARK" => KeyEvent::QEST_MARK,
            "EXCL_MARK" => KeyEvent::EXCL_MARK,
            "SEMI_COLON" => KeyEvent::SEMI_COLON,
            "DBL_QUOTE" => KeyEvent::DBL_QUOTE,
            "SGL_QUOTE" => KeyEvent::SGL_QUOTE,
            "BRKT_OPEN" => KeyEvent::BRKT_OPEN,
            "BRKT_CLOSE" => KeyEvent::BRKT_CLOSE,
            "SLASH" => KeyEvent::SLASH,
            "BACKSLASH" => KeyEvent::BACKSLASH,
            "ELLIPSIS" => KeyEvent::ELLIPSIS,
            "ENTER" => KeyEvent::ENTER,
            "SPACE" => KeyEvent::SPACE,
            "MINUS" => KeyEvent::MINUS,
            "EQUAL" => KeyEvent::EQUAL,
            "UP" => KeyEvent::UP,
            "DOWN" => KeyEvent::DOWN,
            "LEFT" => KeyEvent::LEFT,
            "RIGHT" => KeyEvent::RIGHT,
            "SHIFT" => KeyEvent::SHIFT,
            "CTRL" => KeyEvent::CTRL,
            "ALT" => KeyEvent::ALT,
            _ => Self::default(),
        }
    }
}

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
                Ok(18) => msg.command = mod_CommandToFcitx::OneOfcommand::update_preedit(r.read_message::<UpdatePreedit>(bytes)?),
                Ok(26) => msg.command = mod_CommandToFcitx::OneOfcommand::update_lt(r.read_message::<UpdateLookuptable>(bytes)?),
                Ok(34) => msg.command = mod_CommandToFcitx::OneOfcommand::update_aux(r.read_message::<UpdateAux>(bytes)?),
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
            mod_CommandToFcitx::OneOfcommand::update_preedit(ref m) => 1 + sizeof_len((m).get_size()),
            mod_CommandToFcitx::OneOfcommand::update_lt(ref m) => 1 + sizeof_len((m).get_size()),
            mod_CommandToFcitx::OneOfcommand::update_aux(ref m) => 1 + sizeof_len((m).get_size()),
            mod_CommandToFcitx::OneOfcommand::None => 0,
    }    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        match self.command {            mod_CommandToFcitx::OneOfcommand::commit_text(ref m) => { w.write_with_tag(10, |w| w.write_message(m))? },
            mod_CommandToFcitx::OneOfcommand::update_preedit(ref m) => { w.write_with_tag(18, |w| w.write_message(m))? },
            mod_CommandToFcitx::OneOfcommand::update_lt(ref m) => { w.write_with_tag(26, |w| w.write_message(m))? },
            mod_CommandToFcitx::OneOfcommand::update_aux(ref m) => { w.write_with_tag(34, |w| w.write_message(m))? },
            mod_CommandToFcitx::OneOfcommand::None => {},
    }        Ok(())
    }
}

pub mod mod_CommandToFcitx {

use super::*;

#[derive(Debug, PartialEq, Clone)]
pub enum OneOfcommand<'a> {
    commit_text(CommitText<'a>),
    update_preedit(UpdatePreedit<'a>),
    update_lt(UpdateLookuptable<'a>),
    update_aux(UpdateAux<'a>),
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
pub struct UpdateAux<'a> {
    pub candidates: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for UpdateAux<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.candidates = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for UpdateAux<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.candidates == "" { 0 } else { 1 + sizeof_len((&self.candidates).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.candidates != "" { w.write_with_tag(10, |w| w.write_string(&**&self.candidates))?; }
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

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct FcitxEvent {
    pub event: KeyEvent,
}

impl<'a> MessageRead<'a> for FcitxEvent {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.event = r.read_enum(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for FcitxEvent {
    fn get_size(&self) -> usize {
        0
        + if self.event == msgs::KeyEvent::NUM_0 { 0 } else { 1 + sizeof_varint(*(&self.event) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.event != msgs::KeyEvent::NUM_0 { w.write_with_tag(8, |w| w.write_enum(*&self.event as i32))?; }
        Ok(())
    }
}

