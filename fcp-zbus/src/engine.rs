use regex::Regex;
use reqwest;
use tokio::sync::{Mutex, MutexGuard};
use zvariant::Value;

use crate::{
    generated::{IBusProxy, PanelProxy},
    ibus_variants::IBusText,
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

#[derive(Debug)]
pub struct Candidates {
    depth: QueryDepth,
    pub candidates: Vec<Candidate>,
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

            // Update preedit.

            // Update UI.

            // Handle the case where there's no character left and we get out of a session.

            // Query for candidates.

            // Update UI.

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
            let new_preedit =
                Self::concate_str_with_ascii_u32(&self.state.clone_last_query().await, keyval);

            let mut shared_preedit = self.state.last_query_mtx().await;
            shared_preedit.clear();
            shared_preedit.push_str(&new_preedit);
            drop(shared_preedit); // Release the lock as soon as we can.

            // Update UI.
            self.panel
                .update_preedit_text(
                    &Value::from(
                        IBusText {
                            text: new_preedit.clone(),
                        }
                        .into_struct(),
                    ),
                    0,
                    true,
                )
                .await
                .expect("Failed to update preedit.");

            // Query for candidates.

            return true;
        }

        return false;
    }

    fn concate_str_with_ascii_u32(s: &String, c: u32) -> String {
        let mut new = s.clone();
        new.push(char::from_u32(c).expect(&format!("Cannot convert u32 {c} to char.")));
        return new;
    }
}
