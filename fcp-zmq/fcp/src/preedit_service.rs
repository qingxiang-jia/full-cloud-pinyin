use std::sync::Arc;

use std::sync::Mutex;

use crate::ims::Req;

struct State {
    preedit: Vec<char>,
}

impl State {
    fn new() -> State {
        State {
            preedit: Vec::new(),
        }
    }
}

pub struct PreeditService {
    zmq: Arc<Mutex<Req>>,
    state: Mutex<State>,
}

impl PreeditService {
    pub fn new(ibus: Arc<Mutex<Req>>) -> PreeditService {
        PreeditService {
            zmq: ibus,
            state: Mutex::new(State::new()),
        }
    }

    pub fn push(&self, c: char) {
        let mut state = self.state.lock().expect("push: Failed to lock state.");
        state.preedit.push(c);
        let preedit: String = state.preedit.iter().cloned().collect();

        drop(state);

        self.zmq
            .lock()
            .expect("push: Failed to lock zmq.")
            .update_preedit(&preedit)
    }

    pub fn pop(&self) -> Option<char> {
        let mut state = self.state.lock().expect("pop: Failed to lock state.");
        let popped = state.preedit.pop();
        let preedit: String = state.preedit.iter().cloned().collect();

        drop(state);

        self.zmq
            .lock()
            .expect("pop: Failed to lock zmq.")
            .update_preedit(&preedit);

        popped
    }

    pub fn to_string(&self) -> String {
        let state = self.state.lock().expect("to_string: Failed to lock state.");
        state.preedit.iter().cloned().collect()
    }

    pub fn clear(&self) {
        let mut state = self.state.lock().expect("clear: Failed to lock state.");
        state.preedit.clear();
    }
}
