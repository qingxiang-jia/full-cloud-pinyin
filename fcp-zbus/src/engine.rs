use regex::Regex;
use reqwest::{self, header::USER_AGENT};
use tokio::sync::Mutex;
use zbus::Connection;

use crate::{ibus_proxy::IBusProxy, ibus_variants::IBusLookupTable, keys::Key};

// Implementation of org.freedesktop.IBus.Engine interface

#[derive(PartialEq)]
enum Intent {
    PageDown,
    PageUp,
    Typing,
}

#[derive(Debug, Clone)]
pub struct Candidate {
    pub word: String,
    annotation: String,
    matched_len: Option<i32>,
}

struct State {
    preedit: String,
    depth: usize,
    session: bool,
    candidates: Vec<Candidate>,
    page: usize,
}

unsafe impl Sync for State {} // State is safe to share between threads

impl State {
    pub fn new() -> Self {
        State {
            preedit: "".to_owned(),
            depth: 0,
            session: false,
            candidates: Vec::new(),
            page: 0,
        }
    }
}

pub struct FcpEngine {
    ibus: IBusProxy,
    http: reqwest::Client,
    re: Regex,
    lt_size: usize,
    levels: Vec<usize>,
    state: Mutex<State>,
}

impl FcpEngine {
    pub fn new(conn: &Connection) -> FcpEngine {
        FcpEngine {
            ibus: IBusProxy::new(&conn),
            http: reqwest::Client::new(),
            re: Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input."),
            levels: vec![11, 21, 41, 81, 161, 321, 641, 1281],
            lt_size: 5,
            state: Mutex::new(State::new()),
        }
    }

    pub async fn on_input(&self, key: Key) -> bool {
        match key {
            Key::A
            | Key::B
            | Key::C
            | Key::D
            | Key::E
            | Key::F
            | Key::G
            | Key::H
            | Key::I
            | Key::J
            | Key::K
            | Key::L
            | Key::M
            | Key::N
            | Key::O
            | Key::P
            | Key::Q
            | Key::R
            | Key::S
            | Key::T
            | Key::U
            | Key::V
            | Key::W
            | Key::X
            | Key::Y
            | Key::Z => self.user_types(key).await,
            Key::_0
            | Key::_1
            | Key::_2
            | Key::_3
            | Key::_4
            | Key::_5
            | Key::_6
            | Key::_7
            | Key::_8
            | Key::_9 => self.user_selects(key).await,
            Key::Comma
            | Key::Period
            | Key::SemiColon
            | Key::Colon
            | Key::SingleQuote
            | Key::DoubleQuote
            | Key::QuestionMark => self.to_full_width(key).await,
            Key::Space
            | Key::Enter
            | Key::Shift
            | Key::Minus
            | Key::Equal
            | Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::Backspace
            | Key::Escape => self.user_controls(key).await,
        }
    }

    pub async fn user_types(&self, key: Key) -> bool {
        let mut state = self.state.lock().await;

        state.session = true;
        let preedit = FcpEngine::concate(
            &state.preedit,
            key.to_char().expect("Key cannot be converted to String."),
        );
        state.preedit = preedit.clone();
        drop(state);

        self.ibus.update_preedit_text(&preedit, 1, true).await;
        self.send_to_ibus(0, self.lt_size, Intent::Typing).await;

        true
    }

    pub async fn user_selects(&self, key: Key) -> bool {
        let cand_label = key.to_usize().expect("Key cannot be converted to a usize.");

        if 1 <= cand_label && cand_label <= self.lt_size {
            let cand_idx = cand_label - 1;
            let cand = self.state.lock().await.candidates[cand_idx].clone();
            self.ibus.commit_text(&cand.word).await;

            // Reset lookup table
            let lt = IBusLookupTable::from_nothing();
            self.ibus.update_lookup_table(lt, false).await;

            // Reset preedit
            self.ibus.update_preedit_text("", 0, false).await;

            // Reset state
            let mut state = self.state.lock().await;
            state.candidates.clear();
            state.depth = 0;
            state.page = 0;
            state.preedit = "".to_owned();
            state.session = false;
            return true;
            // TODO: if matched length is less than the length of preedit, the remaining preedit should be used to make another query.
        } else {
            return false;
        }
    }

    pub async fn to_full_width(&self, key: Key) -> bool {
        println!("begin {:#?}", &key);

        let fw_puctuation = key
            .to_full_width_string()
            .expect("This key cannot be converted to fullwidth string.");

        println!("Committing: {}", &fw_puctuation);

        self.ibus.commit_text(&fw_puctuation).await;
        return true;
    }

    pub async fn user_controls(&self, key: Key) -> bool {
        let mut state = self.state.lock().await;

        if let Key::Shift = key {
            // Reset state
            state.candidates.clear();
            state.depth = 0;
            state.page = 0;
            state.preedit = "".to_owned();
            state.session = false;

            drop(state);

            // Reset preedit
            self.ibus.update_preedit_text("", 0, false).await;

            // Reset lookup table
            let lt = IBusLookupTable::from_nothing();
            self.ibus.update_lookup_table(lt, false).await;

            return true;
        }

        if !state.session {
            return false;
        }

        drop(state);

        match key {
            Key::Space => return self.handle_select(1).await,
            Key::Enter => {
                let mut state = self.state.lock().await;

                let preedit = state.preedit.clone();

                // Reset state
                state.candidates.clear();
                state.depth = 0;
                state.page = 0;
                state.preedit = "".to_owned();
                state.session = false;

                drop(state);

                // Commit preddit as alphabets
                self.ibus.commit_text(&preedit).await;

                // Reset preedit
                self.ibus.update_preedit_text("", 0, false).await;

                // Reset lookup table
                let lt = IBusLookupTable::from_nothing();
                self.ibus.update_lookup_table(lt, false).await;

                true
            }
            Key::Minus => {
                let mut state = self.state.lock().await;

                if state.page == 0 {
                    return false;
                }

                state.page -= 1; // Updated in send_to_ibus
                let start = state.page * self.lt_size;
                let end = start + self.lt_size;

                drop(state);

                self.send_to_ibus(start, end, Intent::PageUp).await;

                true
            }
            Key::Equal => {
                let mut state = self.state.lock().await;

                state.page += 1; // Updated in send_to_ibus
                let start = state.page * self.lt_size;
                let end = start + self.lt_size;

                drop(state);

                self.send_to_ibus(start, end, Intent::PageDown).await;

                true
            }
            Key::Up => return false,    // For now, ingore
            Key::Down => return false,  // For now, ignore
            Key::Left => return false,  // For now, ignore
            Key::Right => return false, // For now, ignore
            Key::Backspace => {
                let popped = self.state.lock().await.preedit.pop();
                if popped.is_none() {
                    let mut state = self.state.lock().await;

                    // Reset state
                    state.candidates.clear();
                    state.depth = 0;
                    state.page = 0;
                    state.session = false;

                    // Reset preedit
                    self.ibus.update_preedit_text("", 0, false).await;

                    // Reset lookup table
                    let lt = IBusLookupTable::from_nothing();
                    self.ibus.update_lookup_table(lt, false).await;

                    return false;
                }

                // Update preedit
                let preedit = self.state.lock().await.preedit.clone();
                self.ibus.update_preedit_text(&preedit, 0, true).await;

                self.send_to_ibus(0, self.lt_size, Intent::Typing).await;

                true
            }
            Key::Escape => {
                let mut state = self.state.lock().await;

                // Reset state
                state.candidates.clear();
                state.depth = 0;
                state.page = 0;
                state.preedit = "".to_owned();
                state.session = false;

                // Reset preedit
                self.ibus.update_preedit_text("", 0, false).await;

                // Reset lookup table
                let lt = IBusLookupTable::from_nothing();
                self.ibus.update_lookup_table(lt, false).await;

                true
            }
            _ => panic!("Invalid control key."),
        }
    }

    async fn handle_select(&self, cand_label: usize) -> bool {
        if 1 <= cand_label && cand_label <= self.lt_size {
            let cand_idx = cand_label - 1;
            let cand = self.state.lock().await.candidates[cand_idx].clone();
            self.ibus.commit_text(&cand.word).await;

            // Reset lookup table
            let lt = IBusLookupTable::from_nothing();
            self.ibus.update_lookup_table(lt, false).await;

            // Reset preedit
            self.ibus.update_preedit_text("", 0, false).await;

            // Reset state
            let mut state = self.state.lock().await;
            state.candidates.clear();
            state.depth = 0;
            state.page = 0;
            state.preedit = "".to_owned();
            state.session = false;
            return true;
            // TODO: if matched length is less than the length of preedit, the remaining preedit should be used to make another query.
        } else {
            return false;
        }
    }

    // Candidates and page are updated as needed. Query for candidates made as needed.
    // start: inclusive, end: exclusive
    async fn send_to_ibus(&self, start: usize, mut end: usize, intent: Intent) {
        let state = self.state.lock().await;
        let preedit = state.preedit.clone();
        let depth = state.depth;
        drop(state);

        if intent == Intent::Typing {
            let max_cand = self.levels[depth];
            let cands = self.query_candidates(&preedit, max_cand).await;

            if end > cands.len() {
                end = cands.len();
            }

            let mut state = self.state.lock().await;
            state.candidates = cands.clone();
            state.page = 0;
            drop(state);

            let lt = IBusLookupTable::from_candidates(&cands[0..end]);
            self.ibus.update_lookup_table(lt, true).await;
        }

        if intent == Intent::PageDown {
            let state = self.state.lock().await;

            if start >= state.candidates.len() || end > state.candidates.len() {
                // Need to query for new candidates
                let mut depth = state.depth + 1;
                if depth >= self.levels.len() {
                    depth = self.levels.len() - 1;
                }
                let max_cands = self.levels[depth];
                drop(state);

                let cands = self.query_candidates(&preedit, max_cands).await;
                self.state.lock().await.candidates = cands;
            } else {
                drop(state);
            }

            let mut state = self.state.lock().await;
            state.page += 1;

            let cands_slice = &state.candidates[start..end];
            let cands = cands_slice.to_vec();

            let lt = IBusLookupTable::from_candidates(&cands);
            self.ibus.update_lookup_table(lt, true).await;
        }

        if intent == Intent::PageUp {
            let mut state = self.state.lock().await;
            state.page -= 1;

            let cands_slice = &state.candidates[start..end];
            let cands = cands_slice.to_vec();

            let lt = IBusLookupTable::from_candidates(&cands);
            self.ibus.update_lookup_table(lt, true).await;
        }
    }

    async fn query_candidates(&self, preedit: &str, depth: usize) -> Vec<Candidate> {
        let json = self.get_candidates_from_net(preedit, depth as i32).await;
        let candidates = self.json_to_candidates(json);
        candidates
    }

    async fn get_candidates_from_net(&self, preedit: &str, depth: i32) -> String {
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

    fn json_to_candidates(&self, s: String) -> Vec<Candidate> {
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

    fn concate(s: &str, c: char) -> String {
        let mut new = s.clone().to_owned();
        new.push(c);
        new
    }
}
