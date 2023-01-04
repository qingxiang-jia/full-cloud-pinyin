use std::{
    path::PathBuf,
    sync::{Arc, Mutex, RwLock},
};

use fcitx5::{Fcitx5FnPtrs, FcitxKey};
use regex::Regex;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use sled;
use std::fs;
use tokio::runtime::Runtime;

use crate::fcitx5::{self, Fcitx5};

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
    rt: Runtime,
    http: reqwest::Client,
    cache: sled::Db,
    last_query: Mutex<String>,
    query_depth: Mutex<QueryDepth>,
    re: Regex,
    sym: ZhCnSymbolHandler,
    fcitx5: RwLock<Fcitx5>,
    in_session: Mutex<bool>,
    session_candidates: Mutex<Option<Vec<Candidate>>>,
    table_size: u8,
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
            rt: Runtime::new().expect("Failed to initialize Tokio runtime."),
            http: reqwest::Client::new(),
            cache: db,
            last_query: Mutex::new("".to_owned()),
            query_depth: Mutex::new(QueryDepth::D1),
            re: Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input."),
            sym: ZhCnSymbolHandler::new(),
            fcitx5: RwLock::new(Fcitx5::new()),
            in_session: false.into(),
            session_candidates: Mutex::new(None),
            table_size: 5,
        }
    }

    pub fn set_fcitx5(&self, fn_ptrs: Fcitx5FnPtrs) {
        self.fcitx5
            .write()
            .expect("Failed to lock fcitx5 in write mode.")
            .set_fn_ptrs(fn_ptrs);
    }

    // Returns whether the key has been handled
    pub fn on_key_press(self: Arc<Fcp>, key: FcitxKey) -> bool {
        let mut in_session_mtx = self.in_session.lock().expect("Failed to lock in_session.");

        match key {
            FcitxKey::Num0
            | FcitxKey::Num1
            | FcitxKey::Num2
            | FcitxKey::Num3
            | FcitxKey::Num4
            | FcitxKey::Num5
            | FcitxKey::Num6
            | FcitxKey::Num7
            | FcitxKey::Num8
            | FcitxKey::Num9 => {
                // Select a candidate by keying in 0-9
                if *in_session_mtx {
                    let idx: u8 = (key as u32 - FcitxKey::Num1 as u32) as u8;
                    if idx < self.table_size {
                        // Get matched length of this selected candidate
                        let current_candidates_mtx = self
                            .session_candidates
                            .lock()
                            .expect("Failed to lock session_candidates.");
                        let current_candidates = current_candidates_mtx
                            .as_ref()
                            .expect("session_candidate is None.");
                        let matched_len = (&current_candidates[idx as usize]).matched_len;
                        // Drop the mutex since later we will lock again to avoid deadlock
                        drop(current_candidates_mtx);
                        // Update preedit
                        let mut shared_preedit =
                            self.last_query.lock().expect("Failed to lock last_query.");
                        if matched_len.is_none() {
                            // Full match (by Google Input Tools' convention)
                            shared_preedit.clear();
                        } else {
                            let len = matched_len.expect("matched_len is None.") as usize;
                            if len >= shared_preedit.len() {
                                shared_preedit.clear();
                            } else {
                                shared_preedit.drain(0..len);
                            }
                        }
                        // Clear session_candidates
                        let mut session_candidates = self
                            .session_candidates
                            .lock()
                            .expect("Failed to lock session_candidates.");
                        *session_candidates = None;
                        // Commit candidate
                        self.fcitx5
                            .read()
                            .expect("Failed to lock fcitx5 in read mode.")
                            .engine_commit(idx as usize);
                        if shared_preedit.is_empty() {
                            *in_session_mtx = false;
                        } else {
                            *in_session_mtx = true;
                            // Request new candidates for the rest of the preedit
                            let preedit = shared_preedit.clone();
                            let async_self = self.clone();
                            self.clone().rt.spawn(async move {
                                // Request new candidates
                                let new_candidates = async_self.query_candidates(&preedit).await;
                                // Make CString array
                                let display_texts = Fcp::candidate_vec_to_str_vec(&new_candidates);
                                async_self
                                    .clone()
                                    .fcitx5
                                    .read()
                                    .expect("Failed to lock fcitx5 in read mode.")
                                    .ui_set_candidates(display_texts);
                                // Reset the lookup table page to the first
                                async_self
                                    .fcitx5
                                    .read()
                                    .expect("Failed to lock fcitx5 in read mode.")
                                    .table_set_page(0);
                                // Set session_candidates
                                let mut session_candidates = async_self
                                    .session_candidates
                                    .lock()
                                    .expect("Failed to lock session_candidates.");
                                *session_candidates = Some(new_candidates);
                            });
                        }
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            FcitxKey::Space => {
                // Select a candidate by Space key
                if *in_session_mtx {
                    // Clear preedit
                    let mut shared_preedit =
                        self.last_query.lock().expect("Failed to lock last_query.");
                    shared_preedit.clear();
                    // Clear session_candidates
                    let mut session_candidates = self
                        .session_candidates
                        .lock()
                        .expect("Failed to lock session_candidates.");
                    *session_candidates = None;
                    // Select candidate
                    self.fcitx5
                        .read()
                        .expect("Failed to lock fcitx5 in read mode.")
                        .engine_commit_candidate_by_fixed_key();
                    *in_session_mtx = false;
                    true
                } else {
                    false
                }
            }
            FcitxKey::Equal => {
                // Go to the next page by keying in the next page keys
                if *in_session_mtx {
                    if self
                        .fcitx5
                        .read()
                        .expect("Failed to lock fcitx5 in read mode.")
                        .table_can_page_up()
                    {
                        self.fcitx5
                            .read()
                            .expect("Failed to lock fcitx5 in read mode.")
                            .table_page_up();
                    } else {
                        let preedit = self
                            .last_query
                            .lock()
                            .expect("Failed to lock last_query.")
                            .clone();
                        let async_self = self.clone();
                        self.clone().rt.spawn(async move {
                            // Request new candidates
                            let new_candidates = async_self.query_candidates(&preedit).await;
                            // Set it to UI
                            let display_texts = Fcp::candidate_vec_to_str_vec(&new_candidates);
                            async_self
                                .fcitx5
                                .read()
                                .expect("Failed to lock fcitx5 in read mode.")
                                .ui_set_candidates(display_texts);
                            // Set session_candidates
                            let mut session_candidates = async_self
                                .session_candidates
                                .lock()
                                .expect("Failed to lock session_candidates.");
                            *session_candidates = Some(new_candidates);
                        });
                    }
                    true
                } else {
                    false
                }
            }
            FcitxKey::Minus => {
                // Go to the previous page by previous page keys
                if *in_session_mtx {
                    self.fcitx5
                        .read()
                        .expect("Failed to lock fcitx5 in read mode.")
                        .table_page_down();
                    true
                } else {
                    // Hanlde special symbol input
                    let sym_to_commit = self.sym.handle(key);
                    if sym_to_commit.len() == 0 {
                        return false;
                    }
                    // Commit that symbol
                    self.fcitx5
                        .read()
                        .expect("Failed to lock fcitx5 in read mode.")
                        .engine_commit_preedit(&sym_to_commit);
                    true
                }
            }
            FcitxKey::Right => {
                // Go to the next candidate by ->
                if *in_session_mtx {
                    self.fcitx5
                        .read()
                        .expect("Failed to lock fcitx5 in read mode.")
                        .table_next();
                    true
                } else {
                    false
                }
            }
            FcitxKey::Left => {
                // Go to the previous candidate by <-
                if *in_session_mtx {
                    self.fcitx5
                        .read()
                        .expect("Failed to lock fcitx5 in read mode.")
                        .table_prev();
                    true
                } else {
                    false
                }
            }
            FcitxKey::BackSpace => {
                // Remove one character from preedit
                if *in_session_mtx {
                    // Update preedit
                    let mut shared_preedit =
                        self.last_query.lock().expect("Failed to lock last_query.");
                    shared_preedit.pop();
                    let preedit = shared_preedit.clone();

                    // Update preedit UI
                    self.fcitx5
                        .read()
                        .expect("Failed to lock fcitx5 in read mode.")
                        .ui_set_preedit(&preedit);

                    // If nothing left, we are out of session
                    if preedit.len() == 0 {
                        // Clear session_candidates
                        let mut session_candidates = self
                            .session_candidates
                            .lock()
                            .expect("Failed to lock session_candidates.");
                        *session_candidates = None;
                        *in_session_mtx = false;
                        // Update UI
                        self.fcitx5
                            .read()
                            .expect("Failed to lock fcitx5 in read mode.")
                            .ui_clear_candidates();
                        return true;
                    }

                    let async_self = self.clone();
                    self.clone().rt.spawn(async move {
                        // Request new candidates
                        let new_candidates = async_self.query_candidates(&preedit).await;
                        // Set candidates to UI
                        let display_texts = Fcp::candidate_vec_to_str_vec(&new_candidates);
                        async_self
                            .fcitx5
                            .read()
                            .expect("Failed to lock fcitx5 in read mode.")
                            .ui_set_candidates(display_texts);
                        // Reset the lookup table page to the first
                        async_self
                            .fcitx5
                            .read()
                            .expect("Failed to lock fcitx5 in read mode.")
                            .table_set_page(0);
                        // Clear session_candidates
                        let mut session_candidates = async_self
                            .session_candidates
                            .lock()
                            .expect("Failed to lock session_candidates.");
                        *session_candidates = Some(new_candidates);
                    });
                    true
                } else {
                    false
                }
            }
            FcitxKey::Return => {
                // Commit buffer as is (i.e., not Chinese)
                if *in_session_mtx {
                    // Clear preedit
                    let mut shared_preedit =
                        self.last_query.lock().expect("Failed to lock last_query.");
                    let preedit = shared_preedit.clone();
                    shared_preedit.clear();
                    // Clear session_candidates
                    let mut session_candidates = self
                        .session_candidates
                        .lock()
                        .expect("Failed to lock session_candidates.");
                    *session_candidates = None;
                    // Set flag
                    *in_session_mtx = false;
                    // Commit preedit
                    self.fcitx5
                        .read()
                        .expect("Failed to lock fcitx5 in read mode.")
                        .engine_commit_preedit(&preedit);
                    // Update UI
                    self.fcitx5
                        .read()
                        .expect("Failed to lock fcitx5 in read mode.")
                        .ui_clear_candidates();
                    
                    true
                } else {
                    false
                }
            }
            FcitxKey::Escape => {
                // Terminate this input session
                if *in_session_mtx {
                    // Clear preedit
                    let mut shared_preedit =
                        self.last_query.lock().expect("Failed to lock last_query.");
                    shared_preedit.clear();
                    // Clear session_candidates
                    let mut session_candidates = self
                        .session_candidates
                        .lock()
                        .expect("Failed to lock session_candidates.");
                    *session_candidates = None;
                    // Set flag
                    *in_session_mtx = false;
                    // Update UI
                    self.fcitx5
                        .read()
                        .expect("Failed to lock fcitx5 in read mode.")
                        .ui_clear_candidates();

                    true
                } else {
                    false
                }
            }
            FcitxKey::A
            | FcitxKey::B
            | FcitxKey::C
            | FcitxKey::D
            | FcitxKey::E
            | FcitxKey::F
            | FcitxKey::G
            | FcitxKey::H
            | FcitxKey::I
            | FcitxKey::J
            | FcitxKey::K
            | FcitxKey::L
            | FcitxKey::M
            | FcitxKey::N
            | FcitxKey::O
            | FcitxKey::P
            | FcitxKey::Q
            | FcitxKey::R
            | FcitxKey::S
            | FcitxKey::T
            | FcitxKey::U
            | FcitxKey::V
            | FcitxKey::W
            | FcitxKey::X
            | FcitxKey::Y
            | FcitxKey::Z
            | FcitxKey::a
            | FcitxKey::b
            | FcitxKey::c
            | FcitxKey::d
            | FcitxKey::e
            | FcitxKey::f
            | FcitxKey::g
            | FcitxKey::h
            | FcitxKey::i
            | FcitxKey::j
            | FcitxKey::k
            | FcitxKey::l
            | FcitxKey::m
            | FcitxKey::n
            | FcitxKey::o
            | FcitxKey::p
            | FcitxKey::q
            | FcitxKey::r
            | FcitxKey::s
            | FcitxKey::t
            | FcitxKey::u
            | FcitxKey::v
            | FcitxKey::w
            | FcitxKey::x
            | FcitxKey::y
            | FcitxKey::z => {
                *in_session_mtx = true;
                // Create new query to get candidates
                let val = key as u32;
                if (FcitxKey::a as u32 <= val && val <= FcitxKey::z as u32)
                    || (FcitxKey::A as u32 <= val && val <= FcitxKey::Z as u32)
                {
                    // Add one character from preedit
                    let user_input =
                        char::from_u32(val).expect("The user input cannot be converted to a char.");
                    // Update preedit
                    let mut shared_preedit =
                        self.last_query.lock().expect("Failed to lock last_query.");
                    shared_preedit.push(user_input);
                    let preedit = shared_preedit.clone();
                    // Update preedit UI
                    self.fcitx5
                        .read()
                        .expect("Failed to lock fcitx5 in read mode.")
                        .ui_set_preedit(&preedit);

                    let async_self = self.clone();
                    self.clone().rt.spawn(async move {
                        // Request new candidates
                        let new_candidates = async_self.query_candidates(&preedit).await;
                        // Update candidates to UI
                        let display_texts = Fcp::candidate_vec_to_str_vec(&new_candidates);
                        async_self
                            .fcitx5
                            .read()
                            .expect("Failed to lock fcitx5 in read mode.")
                            .ui_set_candidates(display_texts);
                        // Reset the lookup table page to the first
                        async_self
                            .fcitx5
                            .read()
                            .expect("Failed to lock fcitx5 in read mode.")
                            .table_set_page(0);
                        // Set session_candidates
                        let mut session_candidates = async_self
                            .session_candidates
                            .lock()
                            .expect("Failed to lock session_candidates.");
                        *session_candidates = Some(new_candidates);
                        // Set flag
                        let mut is_in_session = async_self
                            .in_session
                            .lock()
                            .expect("Failed to lock in_session.");
                        *is_in_session = true;
                    });
                    true
                } else {
                    false
                }
            }
            FcitxKey::Comma
            | FcitxKey::Period
            | FcitxKey::Colon
            | FcitxKey::Backslash
            | FcitxKey::Semicolon
            | FcitxKey::Question
            | FcitxKey::Exclam
            | FcitxKey::QuoteDbl
            | FcitxKey::Apostrophe
            | FcitxKey::AsciiCircum
            | FcitxKey::ParenLeft
            | FcitxKey::ParenRight
            | FcitxKey::Less
            | FcitxKey::Hreater
            | FcitxKey::Underscore
            | FcitxKey::Slash
            | FcitxKey::BracketLeft
            | FcitxKey::BracketRight
            | FcitxKey::BraceLeft
            | FcitxKey::BraceRight
            | FcitxKey::Dollar
            | FcitxKey::Asterisk => {
                if !*in_session_mtx {
                    let sym_to_commit = self.sym.handle(key);
                    if sym_to_commit.len() == 0 {
                        return false;
                    }
                    // Commit that symbol
                    self.fcitx5
                        .read()
                        .expect("Failed to lock fcitx5 in read mode.")
                        .engine_commit_preedit(&sym_to_commit);
                    return true;
                }
                false
            }
            _ => false,
        }
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

        let json_str = self
            .get_candidates_from_network(preedit, depth as i32)
            .await;

        let candidates = self.from_json_str_to_structured(json_str);

        self.save_to_cache(preedit, &candidates, depth);

        candidates
    }

    fn decide_query_depth(&self, preedit: &str) -> QueryDepth {
        let mut last_query = self.last_query.lock().expect("Failed to lock last_query.");
        let mut depth = self
            .query_depth
            .lock()
            .expect("Failed to lock query_depth.");
        if last_query.eq(preedit) {
            match *depth {
                QueryDepth::D1 => *depth = QueryDepth::D2,
                QueryDepth::D2 => *depth = QueryDepth::D3,
                QueryDepth::D3 => *depth = QueryDepth::D4,
                QueryDepth::D4 => *depth = QueryDepth::D5,
                QueryDepth::D5 => *depth = QueryDepth::D6,
                QueryDepth::D6 => *depth = QueryDepth::D7,
                QueryDepth::D7 => *depth = QueryDepth::D8,
                QueryDepth::D8 => *depth = QueryDepth::D8,
            }
        } else {
            *last_query = preedit.to_owned();
            *depth = QueryDepth::D1;
        }
        depth.clone()
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

        let resp = self
            .http
            .get(url)
            .header(
                USER_AGENT,
                "Mozilla/5.0 (X11; Linux x86_64; rv:106.0) Gecko/20100101 Firefox/106.0",
            )
            .send()
            .await
            .expect("Network problems when making the request.");

        resp.text()
            .await
            .expect("Network problem when getting the text from the response.")
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

    fn candidate_vec_to_str_vec<'a>(candidates: &'a Vec<Candidate>) -> Vec<&'a String> {
        let strs_only: Vec<&'a String> =
            candidates.iter().map(|candidate| &candidate.word).collect();
        strs_only
    }
}

pub struct ZhCnSymbolHandler {
    quote_1_open: Mutex<bool>,
    quote_2_open: Mutex<bool>,
}

impl ZhCnSymbolHandler {
    pub fn new() -> Self {
        ZhCnSymbolHandler {
            quote_1_open: Mutex::new(true),
            quote_2_open: Mutex::new(true),
        }
    }

    pub fn handle(&self, key: FcitxKey) -> String {
        match key {
            FcitxKey::Comma => "，".to_owned(),
            FcitxKey::Period => "。".to_owned(),
            FcitxKey::Colon => "：".to_owned(),
            FcitxKey::Backslash => "、".to_owned(),
            FcitxKey::Semicolon => "；".to_owned(),
            FcitxKey::Question => "？".to_owned(),
            FcitxKey::Exclam => "！".to_owned(),
            FcitxKey::QuoteDbl => {
                if *self
                    .quote_2_open
                    .lock()
                    .expect("Failed to lock quote_2_open")
                {
                    *self
                        .quote_2_open
                        .lock()
                        .expect("Failed to lock quote_2_open") = false;
                    "“".to_owned()
                } else {
                    *self
                        .quote_2_open
                        .lock()
                        .expect("Failed to lock quote_2_open") = true;
                    "”".to_owned()
                }
            }
            FcitxKey::Apostrophe => {
                if *self
                    .quote_1_open
                    .lock()
                    .expect("Failed to lock quote_2_open")
                {
                    *self
                        .quote_1_open
                        .lock()
                        .expect("Failed to lock quote_2_open") = false;
                    "‘".to_owned()
                } else {
                    *self
                        .quote_1_open
                        .lock()
                        .expect("Failed to lock quote_2_open") = true;
                    "’".to_owned()
                }
            }
            FcitxKey::AsciiCircum => "…".to_owned(),
            FcitxKey::ParenLeft => "（".to_owned(),
            FcitxKey::ParenRight => "）".to_owned(),
            FcitxKey::Less => "《".to_owned(),
            FcitxKey::Hreater => "》".to_owned(),
            FcitxKey::Minus => "－".to_owned(),
            FcitxKey::Underscore => "".to_owned(),
            FcitxKey::Slash => "／".to_owned(),
            FcitxKey::BracketLeft => "【".to_owned(),
            FcitxKey::BracketRight => "】".to_owned(),
            FcitxKey::BraceLeft => "｛".to_owned(),
            FcitxKey::Dollar => "￥".to_owned(),
            FcitxKey::Asterisk => "×".to_owned(),
            _ => "".to_owned(),
        }
    }
}
