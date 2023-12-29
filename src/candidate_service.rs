use std::sync::Arc;

use std::sync::Mutex;

use crate::{candidate::Candidate, zmq::FcitxSock};

struct State {
    candidates: Vec<Candidate>,
    page: usize,
}

impl State {
    pub fn new() -> Self {
        State {
            candidates: Vec::new(),
            page: 0,
        }
    }
}

unsafe impl Sync for State {} // State is safe to share between threads

pub struct CandidateService {
    lt_size: usize,
    state: Mutex<State>,
    zmq: Arc<Mutex<FcitxSock>>,
}

impl CandidateService {
    pub fn new(req: Arc<Mutex<FcitxSock>>) -> CandidateService {
        CandidateService {
            lt_size: 5,
            state: Mutex::new(State::new()),
            zmq: req,
        }
    }

    pub fn in_session(&self) -> bool {
        self.state
            .lock()
            .expect("in_session: Failed to lock state.")
            .candidates
            .len()
            != 0
    }

    pub fn set_candidates(&self, candidates: &[Candidate]) {
        let mut state = self
            .state
            .lock()
            .expect("set_candidates: Failed to lock state.");

        state.candidates.clear();
        for candidate in candidates {
            state.candidates.push(candidate.clone());
        }

        let page = state.page;
        let start = 0 + self.lt_size * page; // inclusive
        let end = start + self.lt_size; // exclusive
        let to_show = if state.candidates.len() <= self.lt_size {
            vec![]
        } else {
            state.candidates[start..end]
                .iter()
                .map(|c| c.word.clone())
                .collect()
        };

        drop(state);

        let zmq = self.zmq.lock().unwrap();
        zmq.update_candidates(&to_show);
    }

    pub fn page_into(&self) -> (bool, Option<usize>) {
        let mut state = self.state.lock().expect("page_into: Failed to lock state.");

        state.page += 1;
        let start = 0 + state.page * self.lt_size;
        let end = start + self.lt_size;
        if start >= state.candidates.len() || end > state.candidates.len() {
            return (false, Some(self.lt_size * (state.page + 1))); // (IsEnough, HowManyAtLeastDoWeNeed)
        }
        let to_show: Vec<String> = state.candidates[start..end]
            .iter()
            .map(|c| c.word.clone())
            .collect();

        drop(state);

        self.zmq.lock().unwrap().update_candidates(&to_show);
        return (true, None);
    }

    pub fn page_back(&self) {
        let mut state = self.state.lock().expect("page_back: Failed to lock state.");

        if state.page == 0 {
            return;
        }
        state.page -= 1;
        let start = 0 + state.page * self.lt_size;
        let end = start + self.lt_size;
        let to_show: Vec<String> = state.candidates[start..end]
            .iter()
            .map(|c| c.word.clone())
            .collect();

        drop(state);

        self.zmq.lock().unwrap().update_candidates(&to_show);
    }

    pub fn select(&self, ith: usize) {
        let state = self.state.lock().expect("select: Failed to lock state.");
        let idx = ith - 1 + state.page * self.lt_size;
        let text = state.candidates[idx].word.clone();

        drop(state);

        self.zmq
            .lock()
            .expect("select: Failed to lock zmq.")
            .commit_text(&text);

        self.clear();
    }

    pub fn clear(&self) {
        let mut state = self.state.lock().expect("clear: Failed to lock state.");
        state.candidates.clear();
        state.page = 0;

        drop(state);

        let zmq = self.zmq.lock().unwrap();
        zmq.update_candidates(&vec![]);
    }
}
