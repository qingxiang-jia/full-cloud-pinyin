use std::sync::{Mutex, MutexGuard};

use regex::Regex;
use reqwest;
use zbus::{dbus_interface, Connection};
use zvariant::ObjectPath;

use crate::generated::{IBusProxy, PanelProxy};

// We have three interfaces to implement in order to get a working engine, but only the
// org.freedesktop.IBus.Engine matters in practice.

// Implementation of org.freedesktop.IBus.Factory interface

pub struct FcpFactory {}

#[dbus_interface(name = "org.freedesktop.IBus.Factory")]
impl FcpFactory {
    pub fn create_engine(&self, name: &str) -> ObjectPath {
        println!("create_engine called by IBus.");
        ObjectPath::from_str_unchecked("/org/freedesktop/IBus/Engine/FcPinyin")
    }
}

// Implementation of org.freedesktop.IBus.Service interface

pub struct FcpService {}

#[dbus_interface(name = "org.freedesktop.IBus.Service")]
impl FcpService {
    pub fn destroy(&self) {
        println!("destroy called by IBus.");
    }
}

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
    pub annotation: String,
    pub matched_len: Option<i32>,
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

    pub fn last_query_mtx(&self) -> MutexGuard<String> {
        self.last_query
            .lock()
            .expect("Failed to lock last_query in last_query_mtx().")
    }

    pub fn clone_last_query(&self) -> String {
        self.last_query
            .lock()
            .expect("Failed to lock last_query in clone_last_query().")
            .clone() // Unlock immediately
    }

    pub fn query_depth_mtx(&self) -> MutexGuard<QueryDepth> {
        self.query_depth
            .lock()
            .expect("Failed to lock query_depth in query_depth_mtx().")
    }

    pub fn in_session_mtx(&self) -> MutexGuard<bool> {
        self.in_session
            .lock()
            .expect("Failed to lock in_session in in_session_mtx().")
    }

    pub fn session_candidates_mtx(&self) -> MutexGuard<Option<Vec<Candidate>>> {
        self.session_candidates
            .lock()
            .expect("Failed to lock session_candidate in session_candidates().")
    }
}

pub struct FcpEngine<'a> {
    ibus: IBusProxy<'a>,
    panel: PanelProxy<'a>,
    http: reqwest::Client,
    re: Regex,
    state: State,
}

#[dbus_interface(name = "org.freedesktop.IBus.Engine")]
impl FcpEngine<'static> {
    pub async fn process_key_event(&self, keyval: u32, keycode: u32, state: u32) -> bool {
        if state != 0 {
            // If it's not "pressed" state, do nothing.
            return false;
        }
        println!("keyval: {keyval}, keycode: {keycode}, state: {state}");

        let mut in_session_mtx = self.state.in_session_mtx();

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

            // Update UI.

            // Query for candidates.

            return true;
        }

        return false;
    }
}

pub async fn new_fcp_engine(conn: &Connection) -> FcpEngine<'static> {
    let ibus = IBusProxy::new(&conn)
        .await
        .expect("Failed to create IBusProxy.");

    let panel = PanelProxy::new(&conn)
        .await
        .expect("Failed to create PanelProxy.");

    FcpEngine {
        ibus,
        panel,
        http: reqwest::Client::new(),
        re: Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input."),
        state: State::new(),
    }
}
