use std::sync::Mutex;

use zbus::Connection;

use crate::keys::Key;

use super::{candidate_service::CandidateService, cloud_pinyin_client::CloudPinyinClient, symbol_service::SymbolService, number_service::NumberService};

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
    ss: SymbolService,
    ns: NumberService,
    client: CloudPinyinClient,
    level: Vec<usize>,
}

impl Dispatcher {
    pub fn new(conn: &Connection) -> Dispatcher {
        Dispatcher {
            state: Mutex::new(State::new()),
            cs: CandidateService::new(conn),
            ss: SymbolService::new(conn),
            ns: NumberService::new(conn),
            client: CloudPinyinClient::new(),
            level: vec![11, 21, 41, 81, 161, 321, 641, 1281],
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
            | Key::_9 => {
                if self.cs.in_session() {
                    return self.handle_select(key).await;
                } else {
                    !unimplemented!()
                }
            }
            Key::Comma
            | Key::Period
            | Key::SemiColon
            | Key::Colon
            | Key::SingleQuote
            | Key::DoubleQuote
            | Key::QuestionMark => {
                self.ss.handle_symbol(key).await;
                return true;
            },
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
    }

    pub async fn handle_pinyin(&self, key: Key) -> bool {
        let c = key.to_char().expect("A-Z cannot be converted to a char.");

        let mut state = self.state.lock().expect("Failed to lock state.");
        state.preedit.push(c);
        let preedit: String = state.preedit.iter().cloned().collect();

        drop(state);

        let candidates = self.client.query_candidates(&preedit, self.level[0]).await;

        self.cs.set_candidates(&candidates).await;

        true
    }

    pub async fn handle_select(&self, key: Key) -> bool {
        !unimplemented!()
    }

    pub async fn handle_control(&self, key: Key) -> bool {
        !unimplemented!()
    }
}
