use std::sync::{Arc, Mutex};

use crate::{
    candidate_service::CandidateService, cloud_pinyin_client::CloudPinyinClient, ims::Req,
    keys::Key, number_service::NumberService, preedit_service::PreeditService,
    symbol_service::SymbolService,
};

pub struct Dispatcher {
    client: CloudPinyinClient,
    candidate_svc: CandidateService,
    preedit_svc: PreeditService,
    symbol_svc: SymbolService,
    number_svc: NumberService,
    zmq: Arc<Mutex<Req>>,
    level: Vec<usize>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        let req: Arc<Mutex<Req>> = Arc::new(Mutex::new(Req::new("tcp://127.0.0.1:8086")));
        Dispatcher {
            client: CloudPinyinClient::new(),
            candidate_svc: CandidateService::new(req.clone()),
            preedit_svc: PreeditService::new(req.clone()),
            symbol_svc: SymbolService::new(req.clone()),
            number_svc: NumberService::new(req.clone()),
            zmq: req.clone(),
            level: vec![11, 21, 41, 81, 161, 321, 641, 1281],
        }
    }

    pub async fn on_input(&self, key: Key) -> bool {
        match key {
            Key::a
            | Key::b
            | Key::c
            | Key::d
            | Key::e
            | Key::f
            | Key::g
            | Key::h
            | Key::i
            | Key::j
            | Key::k
            | Key::l
            | Key::m
            | Key::n
            | Key::o
            | Key::p
            | Key::q
            | Key::r
            | Key::s
            | Key::t
            | Key::u
            | Key::v
            | Key::w
            | Key::x
            | Key::y
            | Key::z => return self.handle_pinyin(key).await,
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
                if self.candidate_svc.in_session() {
                    return self.handle_select(key);
                } else {
                    self.number_svc.handle_number(key);
                    return true;
                }
            }
            Key::Comma
            | Key::Period
            | Key::SemiColon
            | Key::Colon
            | Key::SingleQuote
            | Key::DoubleQuote
            | Key::BracketOpen
            | Key::BracketClose
            | Key::QuestionMark
            | Key::BackSlash
            | Key::ExclamationMark
            | Key::Ellipsis => {
                self.symbol_svc.handle_symbol(key);
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
            Key::Shift | Key::Ctrl | Key::Alt => panic!("Unexpected control keys received."),
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
            | Key::Z => panic!("We do not handle uppercase letters."),
        }
    }

    pub async fn handle_pinyin(&self, key: Key) -> bool {
        let c = key.to_char().expect("A-Z cannot be converted to a char.");

        self.preedit_svc.push(c);
        let preedit = self.preedit_svc.to_string();

        let candidates = self.client.query_candidates(&preedit, self.level[0]).await;

        self.candidate_svc.set_candidates(&candidates);

        true
    }

    pub fn handle_select(&self, key: Key) -> bool {
        self.preedit_svc.clear();

        let i = key.to_usize().expect("Failed to conver the key to usize.");
        self.candidate_svc.select(i);
        self.candidate_svc.clear();

        true
    }

    pub async fn handle_control(&self, key: Key) -> bool {
        if !self.candidate_svc.in_session() {
            return false;
        }

        match key {
            Key::Space => return self.handle_select(Key::_1),
            Key::Enter => {
                let preedit = self.preedit_svc.to_string();
                self.preedit_svc.clear();
                self.candidate_svc.clear();
                self.zmq
                    .lock()
                    .expect("handle_control: Failed to lock zmq.")
                    .commit_text(&preedit);

                return true;
            }
            Key::Minus => {
                self.candidate_svc.page_back();

                return true;
            }
            Key::Equal => {
                let (enough, min_needed) = self.candidate_svc.page_into();
                if !enough {
                    let min = min_needed
                        .expect("Not enough to fill lookup table but min_needed is None.");

                    let mut to_load = 0;
                    for qty in &self.level {
                        if qty >= &min {
                            to_load = *qty;
                            break;
                        }
                    }

                    let candidates = self
                        .client
                        .query_candidates(&self.preedit_svc.to_string(), to_load)
                        .await;
                    self.candidate_svc.set_candidates(&candidates);
                }

                return true;
            }
            Key::Up => return false,    // For now, ingore
            Key::Down => return false,  // For now, ignore
            Key::Left => return false,  // For now, ignore
            Key::Right => return false, // For now, ignore
            Key::Backspace => {
                let popped = self.preedit_svc.pop();

                if popped.is_none() {
                    return false;
                }

                let preedit: String = self.preedit_svc.to_string();

                let candidates = self.client.query_candidates(&preedit, self.level[0]).await;

                self.candidate_svc.set_candidates(&candidates);

                return true;
            }
            Key::Escape => {
                self.preedit_svc.clear();
                self.candidate_svc.clear();

                return true;
            }
            _ => panic!("Invalid control key."),
        }
    }
}
