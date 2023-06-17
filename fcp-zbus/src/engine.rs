use regex::Regex;
use reqwest::{self, header::USER_AGENT};
use tokio::sync::{Mutex, MutexGuard};
use zvariant::Value;

use crate::{
    generated::{IBusProxy, PanelProxy},
    ibus_variants::{IBusLookupTable, IBusText},
};

// Implementation of org.freedesktop.IBus.Engine interface

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone)]
pub struct Candidate {
    pub word: String,
    annotation: String,
    matched_len: Option<i32>,
}

struct State {
    last_query: Mutex<String>,
    query_depth: Mutex<QueryDepth>,
    in_session: Mutex<bool>,
    session_candidates: Mutex<Option<Vec<Candidate>>>,
    table_size: u8,
}

unsafe impl Sync for State {} // State is safe to share between threads

impl State {
    pub fn new() -> Self {
        State {
            last_query: Mutex::new("".to_owned()),
            query_depth: Mutex::new(QueryDepth::D1),
            in_session: Mutex::new(false),
            session_candidates: Mutex::new(None),
            table_size: 5,
        }
    }

    pub async fn last_query_mtx(&self) -> MutexGuard<String> {
        self.last_query.lock().await
    }

    pub async fn clone_last_query(&self) -> String {
        self.last_query.lock().await.clone() // Unlock immediately
    }

    pub async fn query_depth_mtx(&self) -> MutexGuard<QueryDepth> {
        self.query_depth.lock().await
    }

    pub async fn in_session_mtx(&self) -> MutexGuard<bool> {
        self.in_session.lock().await
    }

    pub async fn session_candidates_mtx(&self) -> MutexGuard<Option<Vec<Candidate>>> {
        self.session_candidates.lock().await
    }
}

pub struct FcpEngine<'a> {
    ibus: IBusProxy<'a>,
    panel: PanelProxy<'a>,
    http: reqwest::Client,
    re: Regex,
    state: State,
}

impl<'a> FcpEngine<'a> {
    pub fn new(ibus: IBusProxy<'a>, panel: PanelProxy<'a>) -> FcpEngine<'a> {
        FcpEngine {
            ibus,
            panel,
            http: reqwest::Client::new(),
            re: Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input."),
            state: State::new(),
        }
    }

    pub async fn on_key_press(&self, keyval: u32) -> bool {
        let mut in_session_mtx = self.state.in_session_mtx().await;

        // Select a candidate by entering 0-9.
        if KeyVal::_0 as u32 <= keyval && keyval <= KeyVal::_9 as u32 {
            // If not in session, skip.
            if *in_session_mtx != true {
                return false;
            }

            // Ignore if it's outside the lookup table.

            // If is full match, commit and reset.

            // If is partial match, commit and query.

            return true;
        }

        // Select a candidate by Space key.
        if KeyVal::Space as u32 == keyval {
            // If not in session, skip.
            if *in_session_mtx != true {
                return false;
            }

            // Commit candidate.

            // Should we clear preedit and candidates?

            return true;
        }

        // Page up the lookup table.
        if KeyVal::Equal as u32 == keyval {
            // If not in session, skip.
            if *in_session_mtx != true {
                return false;
            }

            // If lookup table cannot page up, load more candidates.

            return true;
        }

        // Page down the loolup table.
        if KeyVal::Equal as u32 == keyval {
            // If not in session, skip.
            if *in_session_mtx != true {
                return false;
            }

            return true;
        }

        // Go to the next candidate.
        if KeyVal::Right as u32 == keyval || KeyVal::Up as u32 == keyval {
            // If not in session, skip.
            if *in_session_mtx != true {
                return false;
            }

            return true;
        }

        // Go to the previous candidate.
        if KeyVal::Left as u32 == keyval || KeyVal::Down as u32 == keyval {
            // If not in session, skip.
            if *in_session_mtx != true {
                return false;
            }

            return true;
        }

        // Remove one character from preedit.
        if KeyVal::Backspace as u32 == keyval {
            // If not in session, skip.
            if *in_session_mtx != true {
                return false;
            }

            // Compute new preedit.
            let mut new_preedit = self.state.clone_last_query().await;
            new_preedit.pop();

            // Update UI.
            self.panel
                .update_preedit_text(
                    &Value::from(IBusText::new(&new_preedit).into_struct()),
                    0,
                    new_preedit.len() != 0,
                )
                .await
                .expect("Failed to update preedit.");

            // Handle the case where there's no character left and we get out of a session.
            if new_preedit.len() == 0 {
                *in_session_mtx = false;
            }

            // Query for candidates.
            let lookup_table = if new_preedit.len() == 0 {
                IBusLookupTable::new(&Vec::new())
            } else {
                IBusLookupTable::new(&self.query_candidates(&new_preedit).await)
            };

            // Update UI.
            self.panel
                    .update_lookup_table(&Value::new(lookup_table.into_struct()), true)
                    .await
                    .expect("Failed to update lookup table.");

            return true;
        }

        // Commit buffer as English alphabets.
        if KeyVal::Enter as u32 == keyval {
            // If not in session, skip.
            if *in_session_mtx != true {
                return false;
            }

            // Update preedit.

            // Update UI.

            // Query candidates.

            // Update UI.

            return true;
        }

        // Terminate input session.
        if KeyVal::Escape as u32 == keyval {
            // If not in session, skip.
            if *in_session_mtx != true {
                return false;
            }

            // Clear preedit.

            // Set flag.

            // Update UI.

            return true;
        }

        // Create a new query for candidates.
        if KeyVal::A as u32 <= keyval && keyval <= KeyVal::Z as u32 {
            *in_session_mtx = true;

            // Compute new preedit.
            let new_preedit = Self::concate(&self.state.clone_last_query().await, keyval);

            let mut shared_preedit = self.state.last_query_mtx().await;
            shared_preedit.replace_range(.., &new_preedit);

            // Update UI.
            self.panel
                .update_preedit_text(
                    &Value::from(IBusText::new(&new_preedit).into_struct()),
                    0,
                    true,
                )
                .await
                .expect("Failed to update preedit.");

            // Query for candidates.
            let candidates = self.query_candidates(&new_preedit).await;

            // Update UI
            let lookup_table = IBusLookupTable::new(&candidates);
            self.panel
                .update_lookup_table(&Value::new(lookup_table.into_struct()), true)
                .await
                .expect("Failed to update lookup table.");

            return true;
        }

        return false;
    }

    async fn query_candidates(&self, preedit: &str) -> Vec<Candidate> {
        let depth = self.decide_n_update_depth(preedit).await;
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

    async fn decide_n_update_depth(&self, preedit: &str) -> QueryDepth {
        let mut last_query = self.state.last_query_mtx().await;
        let mut depth = self.state.query_depth_mtx().await;
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

    fn concate(s: &String, c: u32) -> String {
        let mut new = s.clone();
        new.push(char::from_u32(c).expect(&format!("Cannot convert u32 {c} to char.")));
        return new;
    }
}
