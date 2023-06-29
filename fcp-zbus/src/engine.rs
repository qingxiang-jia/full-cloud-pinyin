use regex::Regex;
use reqwest::{self, header::USER_AGENT};
use tokio::sync::Mutex;
use zbus::Connection;

use crate::{ibus_proxy::IBusProxy, ibus_variants::IBusLookupTable};

// Implementation of org.freedesktop.IBus.Engine interface

#[allow(dead_code)]
enum KeyVal {
    A = 97,
    B = 98,
    C = 99,
    D = 100,
    E = 101,
    F = 102,
    G = 103,
    H = 104,
    I = 105,
    J = 106,
    K = 107,
    L = 108,
    M = 109,
    N = 110,
    O = 111,
    P = 112,
    Q = 113,
    R = 114,
    S = 115,
    T = 116,
    U = 117,
    V = 118,
    W = 119,
    X = 120,
    Y = 121,
    Z = 122,
    Space = 32,
    Enter = 65293,
    Shift = 65505,
    Minus = 45,
    Equal = 61,
    Up = 65362,
    Down = 65364,
    Left = 65361,
    Right = 65363,
    Backspace = 65288,
    Escape = 65307,
    _0 = 48,
    _1 = 49,
    _2 = 50,
    _3 = 51,
    _4 = 52,
    _5 = 53,
    _6 = 54,
    _7 = 55,
    _8 = 56,
    _9 = 57,
}

impl KeyVal {
    fn from_u32(num: u32) -> KeyVal {
        match num {
            97 => KeyVal::A,
            98 => KeyVal::B,
            99 => KeyVal::C,
            100 => KeyVal::D,
            101 => KeyVal::E,
            102 => KeyVal::F,
            103 => KeyVal::G,
            104 => KeyVal::H,
            105 => KeyVal::I,
            106 => KeyVal::J,
            107 => KeyVal::K,
            108 => KeyVal::L,
            109 => KeyVal::M,
            110 => KeyVal::N,
            111 => KeyVal::O,
            112 => KeyVal::P,
            113 => KeyVal::Q,
            114 => KeyVal::R,
            115 => KeyVal::S,
            116 => KeyVal::T,
            117 => KeyVal::U,
            118 => KeyVal::V,
            119 => KeyVal::W,
            120 => KeyVal::X,
            121 => KeyVal::Y,
            122 => KeyVal::Z,
            32 => KeyVal::Space,
            65293 => KeyVal::Enter,
            65505 => KeyVal::Shift,
            45 => KeyVal::Minus,
            61 => KeyVal::Equal,
            65362 => KeyVal::Up,
            65364 => KeyVal::Down,
            65361 => KeyVal::Left,
            65363 => KeyVal::Right,
            65288 => KeyVal::Backspace,
            65307 => KeyVal::Escape,
            48 => KeyVal::_0,
            49 => KeyVal::_1,
            50 => KeyVal::_2,
            51 => KeyVal::_3,
            52 => KeyVal::_4,
            53 => KeyVal::_5,
            54 => KeyVal::_6,
            55 => KeyVal::_7,
            56 => KeyVal::_8,
            57 => KeyVal::_9,
            _ => panic!("Invalid u32 value cannot be converted to a KeyVal."),
        }
    }
}

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

    pub async fn on_key_press(&self, keyval: u32) -> bool {
        if KeyVal::A as u32 <= keyval && keyval <= KeyVal::Z as u32 {
            return self.handle_typing(keyval).await;
        }
        if KeyVal::_0 as u32 <= keyval && keyval <= KeyVal::_9 as u32 {
            return self.handle_select((keyval - 48) as usize).await;
        }
        if KeyVal::Space as u32 == keyval
            || KeyVal::Enter as u32 == keyval
            || KeyVal::Minus as u32 == keyval
            || KeyVal::Equal as u32 == keyval
            || KeyVal::Up as u32 == keyval
            || KeyVal::Down as u32 == keyval
            || KeyVal::Left as u32 == keyval
            || KeyVal::Right as u32 == keyval
            || KeyVal::Backspace as u32 == keyval
            || KeyVal::Escape as u32 == keyval
        {
            return self.handle_control(KeyVal::from_u32(keyval)).await;
        }

        return false;
    }

    async fn handle_typing(&self, keyval: u32) -> bool {
        let mut state = self.state.lock().await;
        state.session = true;
        let preedit = FcpEngine::concate(&state.preedit, keyval);
        state.preedit = preedit.clone();
        drop(state);

        self.send_to_ibus(0, self.lt_size, Intent::Typing).await;

        true
    }

    async fn handle_control(&self, key: KeyVal) -> bool {
        if !self.state.lock().await.session {
            return false;
        }

        match key {
            KeyVal::Space => return self.handle_select(1).await,
            KeyVal::Enter => {
                let mut state = self.state.lock().await;

                // Reset state
                state.candidates.clear();
                state.depth = 0;
                state.page = 0;
                state.preedit = "".to_owned();
                state.session = false;

                let preedit = state.preedit.clone();

                drop(state);

                // Commit preddit as alphabets
                self.ibus.commit_text(&preedit).await;

                // Reset lookup table
                let lt = IBusLookupTable::from_nothing();
                self.ibus.update_lookup_table(lt, false).await;

                true
            }
            KeyVal::Minus => {
                let mut page = self.state.lock().await.page;

                if page == 0 {
                    return false;
                }

                page -= 1; // Updated in send_to_ibus
                let start = page * self.lt_size;
                let end = start + self.lt_size;

                self.send_to_ibus(start, end, Intent::PageUp).await;

                true
            }
            KeyVal::Equal => {
                let mut page = self.state.lock().await.page;

                page += 1; // Updated in send_to_ibus
                let start = page * self.lt_size;
                let end = start + self.lt_size;

                self.send_to_ibus(start, end, Intent::PageDown).await;

                true
            }
            KeyVal::Up => return false,    // For now, ingore
            KeyVal::Down => return false,  // For now, ignore
            KeyVal::Left => return false,  // For now, ignore
            KeyVal::Right => return false, // For now, ignore
            KeyVal::Backspace => {
                let popped = self.state.lock().await.preedit.pop();
                if popped.is_none() {
                    let mut state = self.state.lock().await;

                    // Reset state
                    state.candidates.clear();
                    state.depth = 0;
                    state.page = 0;
                    state.session = false;

                    // Reset lookup table
                    let lt = IBusLookupTable::from_nothing();
                    self.ibus.update_lookup_table(lt, false).await;

                    return false;
                }

                self.send_to_ibus(0, self.lt_size, Intent::Typing).await;

                true
            }
            KeyVal::Escape => {
                let mut state = self.state.lock().await;

                // Reset state
                state.candidates.clear();
                state.depth = 0;
                state.page = 0;
                state.preedit = "".to_owned();
                state.session = false;

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

    fn concate(s: &String, c: u32) -> String {
        let mut new = s.clone();
        new.push(char::from_u32(c).expect(&format!("Cannot convert u32 {c} to char.")));
        return new;
    }
}
