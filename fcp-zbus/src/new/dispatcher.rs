use std::sync::Mutex;

use zbus::Connection;

use crate::keys::Key;

use super::candidate_service::CandidateService;


struct State {
    preedit: Vec<char>,
}

impl State {
    pub fn new() -> State {
        State {
            preedit: Vec::new(),
        }
    }
}

unsafe impl Sync for State {} // State is safe to share between threads

pub struct Dispatcher {
    state: Mutex<State>,
    cs: CandidateService,
}

impl Dispatcher {
    pub fn new(conn: &Connection) -> Dispatcher {
        Dispatcher {
            state: Mutex::new(State::new()),
            cs: CandidateService::new(conn),
        }
    }
    
    pub async fn on_input(&self, key: Key, should_reset: bool) {
    }
}