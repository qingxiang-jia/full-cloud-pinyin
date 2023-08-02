use zbus::Connection;

use super::ibus_proxy::IBusProxy;
use crate::{keys::Key, preedit_service::PreeditService};

use super::{
    candidate_service::CandidateService, cloud_pinyin_client::CloudPinyinClient,
    number_service::NumberService, symbol_service::SymbolService,
};

pub struct Dispatcher {
    cs: CandidateService,
    ps: PreeditService,
    ss: SymbolService,
    ns: NumberService,
    client: CloudPinyinClient,
    ibus: IBusProxy,
    level: Vec<usize>,
}

impl Dispatcher {
    pub fn new(conn: &Connection) -> Dispatcher {
        Dispatcher {
            cs: CandidateService::new(conn),
            ps: PreeditService::new(conn),
            ss: SymbolService::new(conn),
            ns: NumberService::new(conn),
            client: CloudPinyinClient::new(),
            ibus: IBusProxy::new(conn),
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
                if self.cs.in_session().await {
                    return self.handle_select(key).await;
                } else {
                    self.ns.handle_number(key).await;
                    return true;
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
            }
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

        self.ps.push(c).await;
        let preedit = self.ps.to_string().await;

        let candidates = self.client.query_candidates(&preedit, self.level[0]).await;

        self.cs.set_candidates(&candidates).await;

        true
    }

    pub async fn handle_select(&self, key: Key) -> bool {
        self.ps.clear().await;

        let i = key.to_usize().expect("Failed to conver the key to usize.");
        self.cs.select(i).await;
        self.cs.clear().await;

        true
    }

    pub async fn handle_control(&self, key: Key) -> bool {
        if !self.cs.in_session().await {
            return false;
        }

        match key {
            Key::Space => return self.handle_select(Key::_1).await,
            Key::Enter => {
                let preedit = self.ps.to_string().await;
                self.ps.clear().await;
                self.cs.clear().await;
                self.ibus.commit_text(&preedit).await;

                return true;
            }
            Key::Minus => {
                self.cs.page_back().await;

                return true;
            }
            Key::Equal => {
                self.cs.page_into().await;

                return true;
            }
            Key::Up => return false,    // For now, ingore
            Key::Down => return false,  // For now, ignore
            Key::Left => return false,  // For now, ignore
            Key::Right => return false, // For now, ignore
            Key::Backspace => {
                let popped = self.ps.pop().await;

                if popped.is_none() {
                    self.cs.clear().await;
                    return true;
                }

                let preedit: String = self.ps.to_string().await;

                let candidates = self.client.query_candidates(&preedit, self.level[0]).await;

                self.cs.set_candidates(&candidates).await;

                return true;
            }
            Key::Escape => {
                self.ps.clear().await;
                self.cs.clear().await;

                return true;
            }
            _ => panic!("Invalid control key."),
        }
    }
}
