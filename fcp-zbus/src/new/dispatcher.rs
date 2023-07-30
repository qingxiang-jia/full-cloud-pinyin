use std::sync::Mutex;

use zbus::Connection;

use crate::keys::Key;

use super::{candidate_service::CandidateService, cloud_pinyin_client::CloudPinyinClient};


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
    client: CloudPinyinClient,
}

impl Dispatcher {
    pub fn new(conn: &Connection) -> Dispatcher {
        Dispatcher {
            state: Mutex::new(State::new()),
            cs: CandidateService::new(conn),
            client: CloudPinyinClient::new(),
        }
    }
    
    pub async fn on_input(&self, key: Key, should_reset: bool) -> bool {
        if should_reset {
            self.cs.clear().await;
            return false;
        }

        !unimplemented!()
    }

    pub fn handle_pinyin(key: Key) -> bool {
        !unimplemented!()
    }

    pub fn handle_select(key: Key) -> bool {
        !unimplemented!()
    }

    pub fn handle_symbol(key: Key) -> bool {
        !unimplemented!()
    }

    pub fn handle_control(key: Key) -> bool {
        !unimplemented!()
    }
}