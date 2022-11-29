use std::{cell::Cell, ffi::CString, os::raw::c_char, path::PathBuf, sync::Mutex};

use fcitx5::{UI, Table, Engine, Fcitx5, FcitxKey};
use regex::Regex;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use sled;
use std::fs;

use crate::fcitx5;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum QueryDepth {
    D1 = 11,
    D2 = 21,
    D3 = 41,
    D4 = 81,
    D5 = 161,
    D6 = 321,
    D7 = 641,
    D8 = 1281,
}