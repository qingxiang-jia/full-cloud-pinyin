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

pub struct Fcp {
    http: reqwest::Client,
    cache: sled::Db,
    last_query: Mutex<String>,
    query_depth: Cell<QueryDepth>,
    re: Regex,
    fcitx5: Option<Fcitx5>,
}

impl Fcp {
    pub fn new() -> Self {
        let mut path = match Self::make_config_dir_if_not_already() {
            Ok(path_buf) => path_buf,
            Err(error) => panic!("Failed to create config dir: {:#?}", error),
        };
        path.push("sled_cache");

        let config = sled::Config::default()
            .path(path.as_path())
            .cache_capacity(100 * 1024 * 1024)
            .flush_every_ms(Some(5 * 60 * 1000));

        let db = match config.open() {
            Ok(db) => db,
            Err(error) => panic!("Failed to create cache: {:#?}", error),
        };

        Self {
            http: reqwest::Client::new(),
            cache: db,
            last_query: Mutex::new("".to_owned()),
            query_depth: Cell::new(QueryDepth::D1),
            re: Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input."),
            fcitx5: None,
        }
    }

    pub fn on_key_press(&self, key: FcitxKey) {

    }

    async fn get_candidates_from_network(&self, preedit: &str, depth: i32) {
        let url = format!("https://inputtools.google.com/request?text={}&itc=zh-t-i0-pinyin&num={}&cp=0&cs=1&ie=utf-8&oe=utf-8&app=demopage", preedit, depth);

        let resp = self.http.get(url).header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:106.0) Gecko/20100101 Firefox/106.0",).send();

        // TODO
    }

    fn make_config_dir_if_not_already() -> std::io::Result<PathBuf> {
        let mut path = home::home_dir().expect("Failed to get home path.");
        path.push(".config");
        path.push("fcpinyin/");
        let result = match fs::create_dir_all(path.as_path()) {
            Ok(()) => Ok(path),
            Err(error) => Err(error),
        };
        result
    }
}