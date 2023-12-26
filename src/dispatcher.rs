use std::sync::{Arc, Mutex};

use crate::{
    candidate_service::CandidateService, cloud_pinyin_client::CloudPinyinClient, ims::Req,
    keys::FcitxKeySym, number_service::NumberService, preedit_service::PreeditService,
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

    pub async fn on_input(&self, key: FcitxKeySym) -> bool {
        match key {
            FcitxKeySym::a
            | FcitxKeySym::b
            | FcitxKeySym::c
            | FcitxKeySym::d
            | FcitxKeySym::e
            | FcitxKeySym::f
            | FcitxKeySym::g
            | FcitxKeySym::h
            | FcitxKeySym::i
            | FcitxKeySym::j
            | FcitxKeySym::k
            | FcitxKeySym::l
            | FcitxKeySym::m
            | FcitxKeySym::n
            | FcitxKeySym::o
            | FcitxKeySym::p
            | FcitxKeySym::q
            | FcitxKeySym::r
            | FcitxKeySym::s
            | FcitxKeySym::t
            | FcitxKeySym::u
            | FcitxKeySym::v
            | FcitxKeySym::w
            | FcitxKeySym::x
            | FcitxKeySym::y
            | FcitxKeySym::z => return self.handle_pinyin(key).await,
            FcitxKeySym::Num0
            | FcitxKeySym::Num1
            | FcitxKeySym::Num2
            | FcitxKeySym::Num3
            | FcitxKeySym::Num4
            | FcitxKeySym::Num5
            | FcitxKeySym::Num6
            | FcitxKeySym::Num7
            | FcitxKeySym::Num8
            | FcitxKeySym::Num9 => {
                if self.candidate_svc.in_session() {
                    return self.handle_select(key);
                } else {
                    self.number_svc.handle_number(key);
                    return true;
                }
            }
            FcitxKeySym::Comma
            | FcitxKeySym::Period
            | FcitxKeySym::Semicolon
            | FcitxKeySym::Colon
            | FcitxKeySym::LeftSingleQuoteMark
            | FcitxKeySym::RightSingleQuoteMark
            | FcitxKeySym::DoubleQuote
            | FcitxKeySym::BracketLeft
            | FcitxKeySym::BracketRight
            | FcitxKeySym::Question
            | FcitxKeySym::Backslash
            | FcitxKeySym::Exclam
            | FcitxKeySym::Ellipsis => {
                self.symbol_svc.handle_symbol(key);
                return true;
            }
            FcitxKeySym::Space
            | FcitxKeySym::Return
            | FcitxKeySym::Minus
            | FcitxKeySym::Equal
            | FcitxKeySym::Up
            | FcitxKeySym::Down
            | FcitxKeySym::Left
            | FcitxKeySym::Right
            | FcitxKeySym::BackSpace
            | FcitxKeySym::Escape => return self.handle_control(key).await,
            FcitxKeySym::ShiftL | FcitxKeySym::ControlR | FcitxKeySym::AltL => return false,
            FcitxKeySym::A
            | FcitxKeySym::B
            | FcitxKeySym::C
            | FcitxKeySym::D
            | FcitxKeySym::E
            | FcitxKeySym::F
            | FcitxKeySym::G
            | FcitxKeySym::H
            | FcitxKeySym::I
            | FcitxKeySym::J
            | FcitxKeySym::K
            | FcitxKeySym::L
            | FcitxKeySym::M
            | FcitxKeySym::N
            | FcitxKeySym::O
            | FcitxKeySym::P
            | FcitxKeySym::Q
            | FcitxKeySym::R
            | FcitxKeySym::S
            | FcitxKeySym::T
            | FcitxKeySym::U
            | FcitxKeySym::V
            | FcitxKeySym::W
            | FcitxKeySym::X
            | FcitxKeySym::Y
            | FcitxKeySym::Z => self.commit(key),
            _ => false,
        }
    }

    pub async fn handle_pinyin(&self, key: FcitxKeySym) -> bool {
        let c = key.to_char().expect("A-Z cannot be converted to a char.");

        self.preedit_svc.push(c);
        let preedit = self.preedit_svc.to_string();

        let candidates = self.client.query_candidates(&preedit, self.level[0]).await;

        self.candidate_svc.set_candidates(&candidates);

        true
    }

    pub fn handle_select(&self, key: FcitxKeySym) -> bool {
        self.preedit_svc.clear();

        let i = key.to_usize().expect("Failed to conver the key to usize.");
        self.candidate_svc.select(i);
        self.candidate_svc.clear();

        true
    }

    pub async fn handle_control(&self, key: FcitxKeySym) -> bool {
        if !self.candidate_svc.in_session() {
            return false;
        }

        match key {
            FcitxKeySym::Space => return self.handle_select(FcitxKeySym::Num1),
            FcitxKeySym::Return => {
                let preedit = self.preedit_svc.to_string();
                self.preedit_svc.clear();
                self.candidate_svc.clear();
                self.zmq
                    .lock()
                    .expect("handle_control: Failed to lock zmq.")
                    .commit_text(&preedit);

                return true;
            }
            FcitxKeySym::Minus => {
                self.candidate_svc.page_back();

                return true;
            }
            FcitxKeySym::Equal => {
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
            FcitxKeySym::Up => return false,    // For now, ingore
            FcitxKeySym::Down => return false,  // For now, ignore
            FcitxKeySym::Left => return false,  // For now, ignore
            FcitxKeySym::Right => return false, // For now, ignore
            FcitxKeySym::BackSpace => {
                let popped = self.preedit_svc.pop();

                if popped.is_none() {
                    return false;
                }

                let preedit: String = self.preedit_svc.to_string();

                let candidates = self.client.query_candidates(&preedit, self.level[0]).await;

                self.candidate_svc.set_candidates(&candidates);

                return true;
            }
            FcitxKeySym::Escape => {
                self.preedit_svc.clear();
                self.candidate_svc.clear();

                return true;
            }
            _ => panic!("Invalid control key."),
        }
    }

    pub fn commit(&self, key: FcitxKeySym) -> bool {
        let letter = key.to_char();
        if letter.is_some() {
            let letter = String::from(letter.unwrap());
            self.zmq
                .lock()
                .expect("handle_control: Failed to lock zmq.")
                .commit_text(&letter);
            return true;
        } else {
            return false;
        }
    }
}
