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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub word: String,
    pub annotation: String,
    pub matched_len: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidates {
    depth: QueryDepth,
    candidates: Vec<Candidate>,
}

pub struct Fcp {
    http: reqwest::Client,
    cache: sled::Db,
    last_query: Mutex<String>,
    query_depth: Cell<QueryDepth>,
    re: Regex,
    fcitx5: Cell<Option<Fcitx5>>,
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
            fcitx5: None.into(),
        }
    }

    pub fn set_fcitx5(&self, fcitx5: Fcitx5) {
        self.fcitx5.set(Some(fcitx5));
    }

    pub fn on_key_press(&self, key: FcitxKey) {
        // TODO
    }

    async fn query_candidates(&self, preedit: &str) -> Vec<Candidate> {
        let depth = self.decide_query_depth(preedit);
        let candidates = self.get_candidates(preedit, depth).await;
        candidates
    }

    async fn get_candidates(&self, preedit: &str, depth: QueryDepth) -> Vec<Candidate> {
        let cached = self.get_from_cache(preedit, depth);
        if cached.is_some() {
            return cached.expect("Cached returns None.").candidates;
        }

        let json_str = self.get_candidates_from_network(preedit, depth as i32).await;

        let candidates = self.from_json_str_to_structured(json_str);

        self.save_to_cache(preedit, &candidates, depth);

        candidates
    }

    fn decide_query_depth(&self, preedit: &str) -> QueryDepth {
        let mut last_query = self.last_query.lock().expect("Failed to lock last_query.");
        if last_query.eq(preedit) {
            match self.query_depth.get() {
                QueryDepth::D1 => self.query_depth.set(QueryDepth::D2),
                QueryDepth::D2 => self.query_depth.set(QueryDepth::D3),
                QueryDepth::D3 => self.query_depth.set(QueryDepth::D4),
                QueryDepth::D4 => self.query_depth.set(QueryDepth::D5),
                QueryDepth::D5 => self.query_depth.set(QueryDepth::D6),
                QueryDepth::D6 => self.query_depth.set(QueryDepth::D7),
                QueryDepth::D7 => self.query_depth.set(QueryDepth::D8),
                QueryDepth::D8 => self.query_depth.set(QueryDepth::D8),
            }
        } else {
            *last_query = preedit.to_owned();
            self.query_depth.set(QueryDepth::D1);
        }
        self.query_depth.get()
    }

    fn get_from_cache(&self, preedit: &str, depth: QueryDepth) -> Option<Candidates> {
        let has_key = self.cache.contains_key(preedit).expect(&format!(
            "Cache failed when trying get whether {} exists.",
            preedit
        ));

        if has_key {
            let cached = self
                .cache
                .get(preedit)
                .expect(&format!(
                    "Error occured when getting cached value for {}",
                    preedit
                ))
                .expect(&format!("The cached value for {} doesn't exist.", preedit));

            let mut deserialized: Candidates =
                bincode::deserialize(&cached).expect("The cached value cannot be deserialized.");

            if deserialized.depth > depth {
                deserialized.candidates.truncate(depth as usize);
            }

            if deserialized.depth >= depth {
                Some(deserialized)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn save_to_cache(&self, preedit: &str, candidates: &Vec<Candidate>, depth: QueryDepth) {
        let to_be_saved = Candidates {
            depth,
            candidates: candidates.clone(),
        };
        let serialized = match bincode::serialize(&to_be_saved) {
            Ok(data) => data,
            Err(error) => panic!("Failed to serialize: {:#?}", error),
        };

        _ = self.cache.insert(preedit, serialized);
    }

    async fn get_candidates_from_network(&self, preedit: &str, depth: i32) -> String {
        let url = format!("https://inputtools.google.com/request?text={}&itc=zh-t-i0-pinyin&num={}&cp=0&cs=1&ie=utf-8&oe=utf-8&app=demopage", preedit, depth);

        let resp = self.http.get(url).header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:106.0) Gecko/20100101 Firefox/106.0",).send().await.expect("Network problems when making the request.");

        resp.text().await.expect("Network problem when getting the text from the response.")
    }

    fn from_json_str_to_structured(&self, s: String) -> Vec<Candidate> {
        let mut linear_data: Vec<String> = Vec::new();

        for caps in self.re.captures_iter(&s) {
            for cap in caps.iter() {
                if cap.is_some() {
                    linear_data.push(cap.unwrap().as_str().to_owned());
                }
            }
        }

        let mut colon_pos: Vec<usize> = Vec::new();

        if linear_data[0] != "SUCCESS" {
            println!("Rust: Google returned irregular data:\n{}", s.as_str());
            return Vec::new();
        }

        for i in 0..linear_data.len() {
            if linear_data[i] == ":" {
                colon_pos.push(i);
            }
        }

        let has_matched_len = colon_pos.len() == 4;

        let candidates = &linear_data[2..colon_pos[0] - 1];
        let annotations = &linear_data[colon_pos[0] + 1..colon_pos[1] - 1];

        let matched_len: Option<&[String]>;
        if has_matched_len {
            matched_len = Some(&linear_data[colon_pos[3] + 1..]);
        } else {
            matched_len = None;
        }

        let mut aggregate: Vec<Candidate> = Vec::new();
        for i in 0..candidates.len() {
            aggregate.push(Candidate {
                word: candidates[i].to_owned(),
                annotation: annotations[i].to_owned(),
                matched_len: match matched_len {
                    Some(len) => Some(
                        len[i]
                            .parse::<i32>()
                            .expect("Matched length faield to be parsed to i32."),
                    ),
                    _ => None,
                },
            })
        }

        aggregate
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