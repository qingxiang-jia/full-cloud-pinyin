use std::sync::Mutex;

use zbus::Connection;

use super::{candidate::Candidate, ibus_proxy::IBusProxy, ibus_variants::IBusLookupTable};

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
    
    pub async fn set_candidates(&self, candidates: &[Candidate]) {
        let mut state = self.state.lock().expect("Failed to lock state.");
        
        state.candidates.clear();
        for candidate in candidates {
            state.candidates.push(candidate.clone());
        }

        // IBus
        let page = state.page;
        let start = 0 + self.lt_size * page; // inclusive
        let end = start + self.lt_size; // exclusive
        let to_show = IBusLookupTable::from_candidates(&state.candidates[start..end]);

        drop(state);

        self.ibus.update_lookup_table(to_show, true).await;
    }

    pub fn page_down(&self) {}
    
    pub fn page_up(&self) {}

    pub fn select(&self) {}

    pub fn clear(&self) {}
}
