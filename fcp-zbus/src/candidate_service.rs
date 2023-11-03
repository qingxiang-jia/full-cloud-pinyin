use std::sync::Arc;

use tokio::sync::Mutex;

use super::{candidate::Candidate, ibus_proxy::IBusProxy, ibus_variants::IBusLookupTable};

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
    ibus: Arc<Mutex<IBusProxy>>,
}

impl CandidateService {
    pub fn new(ibus: Arc<Mutex<IBusProxy>>) -> CandidateService {
        CandidateService {
            lt_size: 5,
            state: Mutex::new(State::new()),
            ibus,
        }
    }

    pub async fn in_session(&self) -> bool {
        self.state.lock().await.candidates.len() != 0
    }

    pub async fn set_candidates(&self, candidates: &[Candidate]) {
        let mut state = self.state.lock().await;

        state.candidates.clear();
        for candidate in candidates {
            state.candidates.push(candidate.clone());
        }

        // IBus
        let page = state.page;
        let start = 0 + self.lt_size * page; // inclusive
        let end = start + self.lt_size; // exclusive
        let to_show = if state.candidates.len() <= self.lt_size {
            IBusLookupTable::from_nothing()
        } else {
            IBusLookupTable::from_candidates(&state.candidates[start..end])
        };

        drop(state);

        self.ibus
            .lock()
            .await
            .update_lookup_table(to_show, true)
            .await;
    }

    pub async fn page_into(&self) -> (bool, Option<usize>) {
        let mut state = self.state.lock().await;

        state.page += 1;
        let start = 0 + state.page * self.lt_size;
        let end = start + self.lt_size;
        if start >= state.candidates.len() || end > state.candidates.len() {
            return (false, Some(self.lt_size * (state.page + 1))); // (IsEnough, HowManyAtLeastDoWeNeed)
        }
        let to_show = IBusLookupTable::from_candidates(&state.candidates[start..end]);

        drop(state);

        self.ibus
            .lock()
            .await
            .update_lookup_table(to_show, true)
            .await;
        return (true, None);
    }

    pub async fn page_back(&self) {
        let mut state = self.state.lock().await;

        if state.page == 0 {
            return;
        }
        state.page -= 1;
        let start = 0 + state.page * self.lt_size;
        let end = start + self.lt_size;
        let to_show = IBusLookupTable::from_candidates(&state.candidates[start..end]);

        drop(state);

        self.ibus
            .lock()
            .await
            .update_lookup_table(to_show, true)
            .await;
    }

    pub async fn select(&self, ith: usize) {
        let state = self.state.lock().await;
        let idx = ith - 1 + state.page * self.lt_size;
        let text = state.candidates[idx].word.clone();

        drop(state);

        self.ibus.lock().await.commit_text(&text).await;

        self.clear().await;
    }

    pub async fn clear(&self) {
        let mut state = self.state.lock().await;
        state.candidates.clear();
        state.page = 0;

        drop(state);

        self.ibus
            .lock()
            .await
            .update_lookup_table(IBusLookupTable::from_nothing(), false)
            .await;
    }
}
