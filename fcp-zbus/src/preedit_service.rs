use tokio::sync::Mutex;
use zbus::Connection;

use super::ibus_proxy::IBusProxy;

struct State {
    preedit: Vec<char>
}

impl State {
    fn new() -> State {
        State {
            preedit: Vec::new(),
        }
    }
}

pub struct PreeditService {
    ibus: IBusProxy,
    state: Mutex<State>,
}

impl PreeditService {
    pub fn new(conn: &Connection) -> PreeditService {
        PreeditService {
            ibus: IBusProxy::new(conn),
            state: Mutex::new(State::new()),
        }
    }

    pub async fn push(&self, c: char) {
        let mut state = self.state.lock().await;
        state.preedit.push(c);
    }

    pub async fn pop(&self) -> Option<char> {
        let mut state = self.state.lock().await;
        state.preedit.pop()
    }

    pub async fn to_string(&self) -> String {
        let state = self.state.lock().await;
        state.preedit.iter().cloned().collect()
    }

    pub async fn clear(&self) {
        let mut state = self.state.lock().await;
        state.preedit.clear();
    }
}
