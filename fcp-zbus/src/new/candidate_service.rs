use std::sync::Mutex;

use zbus::Connection;

use crate::ibus_proxy::IBusProxy;

use super::candidate::Candidate;

struct State {
    session: bool,
    candidates: Vec<Candidate>,
    page: usize,
}

impl State {
    pub fn new() -> Self {
        State {
            session: false,
            candidates: Vec::new(),
            page: 0,
        }
    }
}

unsafe impl Sync for State {} // State is safe to share between threads

pub struct CandidateService {
    lt_size: usize,
    levels: Vec<usize>,
    state: Mutex<State>,
    ibus: IBusProxy,
}

impl CandidateService {
    pub fn new(conn: &Connection) -> CandidateService {
        CandidateService {
            lt_size: 5,
            levels: vec![11, 21, 41, 81, 161, 321, 641, 1281],
            state: Mutex::new(State::new()),
            ibus: IBusProxy::new(&conn),
        }
    }

    pub fn page_up() {}

    pub fn page_down() {}

    pub fn set_candidates() {}

    pub fn select() {}

    pub fn clear() {}
}
