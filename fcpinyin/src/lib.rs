mod ffi;

use std::{cell::Cell, path::PathBuf, sync::Mutex};

use regex::Regex;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use sled;
use std::fs;

#[derive(Debug)]
pub struct FullCloudPinyin {
    http: reqwest::blocking::Client,
    cache: sled::Db,
    last_query: Mutex<String>,
    query_depth: Cell<QueryDepth>,
    re: Regex,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidates {
    depth: QueryDepth,
    candidates: Vec<Candidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub word: String,
    pub annotation: String,
    pub matched_len: Option<i32>,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
enum QueryDepth {
    D1 = 11,
    D2 = 21,
    D3 = 41,
    D4 = 81,
    D5 = 161,
    D6 = 321,
    D7 = 641,
    D8 = 1281,
}

impl FullCloudPinyin {
    pub fn new() -> Self {
        let mut path = match Self::make_config_dir_if_not_already() {
            Ok(path_buf) => path_buf,
            Err(error) => panic!("Failed to create config dir: {:#?}", error),
        };
        path.push("sled_cache");

        let config = sled::Config::default()
            .path(path.as_path())
            .cache_capacity(10 * 1024 * 1024)
            .flush_every_ms(Some(5 * 60 * 1000));

        let db = match config.open() {
            Ok(db) => db,
            Err(error) => panic!("Failed to create cache: {:#?}", error),
        };

        Self {
            http: reqwest::blocking::Client::new(),
            cache: db,
            last_query: Mutex::new("".to_owned()),
            query_depth: Cell::new(QueryDepth::D1),
            re: Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input."),
        }
    }

    pub fn query_candidates(&self, preedit: &str) -> Vec<Candidate> {
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
        return self.get_candidates(preedit, self.query_depth.get());
    }

    fn get_candidates(&self, preedit: &str, depth: QueryDepth) -> Vec<Candidate> {
        if preedit.len() == 0 {
            return Vec::new(); // Otherwise we will get FAILED_TO_PARSE_REQUEST_BODY
        }

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
                println!("HIT {} @{:#?}", preedit, depth);
                return deserialized.candidates;
            }
        }

        println!("MISS {} @{:#?}", preedit, depth);

        let url = format!("https://inputtools.google.com/request?text={}&itc=zh-t-i0-pinyin&num={}&cp=0&cs=1&ie=utf-8&oe=utf-8&app=demopage", preedit, depth as i32);

        let rep = self
            .http
            .get(url)
            .header(
                USER_AGENT,
                "Mozilla/5.0 (X11; Linux x86_64; rv:106.0) Gecko/20100101 Firefox/106.0",
            )
            .send()
            .expect("Network problems.");

        let json_str = rep.text().expect("The data cannot be converted to string.");

        let candidates = self.from_json_str_to_structured(json_str);

        // Save to cache
        let to_be_saved = Candidates {
            depth,
            candidates: candidates.clone(),
        };
        let serialized = match bincode::serialize(&to_be_saved) {
            Ok(data) => data,
            Err(error) => panic!("Failed to serialize: {:#?}", error),
        };

        _ = self.cache.insert(preedit, serialized);

        candidates
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
