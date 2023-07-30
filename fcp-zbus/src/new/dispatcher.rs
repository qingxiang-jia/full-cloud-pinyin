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

        match key {
            Key::A
            | Key::B
            | Key::C
            | Key::D
            | Key::E
            | Key::F
            | Key::G
            | Key::H
            | Key::I
            | Key::J
            | Key::K
            | Key::L
            | Key::M
            | Key::N
            | Key::O
            | Key::P
            | Key::Q
            | Key::R
            | Key::S
            | Key::T
            | Key::U
            | Key::V
            | Key::W
            | Key::X
            | Key::Y
            | Key::Z => return self.handle_pinyin(key).await,
            Key::_0
            | Key::_1
            | Key::_2
            | Key::_3
            | Key::_4
            | Key::_5
            | Key::_6
            | Key::_7
            | Key::_8
            | Key::_9 => return self.handle_select(key).await,
            Key::Comma
            | Key::Period
            | Key::SemiColon
            | Key::Colon
            | Key::SingleQuote
            | Key::DoubleQuote
            | Key::QuestionMark => return self.handle_symbol(key).await,
            Key::Space
            | Key::Enter
            | Key::Minus
            | Key::Equal
            | Key::Up
            | Key::Down
            | Key::Left
            | Key::Right
            | Key::Backspace
            | Key::Escape => return self.handle_control(key).await,
        }

        !unimplemented!()
    }

    pub async fn handle_pinyin(&self, key: Key) -> bool {
        !unimplemented!()
    }

    pub async fn handle_select(&self, key: Key) -> bool {
        !unimplemented!()
    }

    pub async fn handle_symbol(&self, key: Key) -> bool {
        !unimplemented!()
    }

    pub async fn handle_control(&self, key: Key) -> bool {
        !unimplemented!()
    }
}