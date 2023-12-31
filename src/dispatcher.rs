use std::sync::{Arc, Mutex};

use crate::{
    candidate_service::CandidateService,
    cloud_pinyin_client::CloudPinyinClient,
    keys::FcitxKeySym,
    number_service::NumberService,
    preedit_service::PreeditService,
    symbol_service::SymbolService,
    zmq::{Client, Server},
};

pub struct Dispatcher {
    client: CloudPinyinClient,
    candidate_svc: CandidateService,
    preedit_svc: PreeditService,
    symbol_svc: SymbolService,
    number_svc: NumberService,
    zmq: Arc<Mutex<Client>>,
    level: Vec<usize>,
}

impl Dispatcher {
    pub fn new() -> Dispatcher {
        let req: Arc<Mutex<Client>> = Arc::new(Mutex::new(Client::new("tcp://127.0.0.1:8086")));
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

    // True if key is accepted; false otherwise.
    pub async fn on_input(&self, key: FcitxKeySym, sock: &Server) {
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
            | FcitxKeySym::z => {
                // Tells the bridge that we accept this event.
                sock.send(true);
                // Work on getting the candidates.
                self.handle_pinyin(key).await;
            }
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
                sock.send(true);
                if self.candidate_svc.in_session() {
                    self.handle_select(key);
                } else {
                    self.number_svc.handle_number(key);
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
            | FcitxKeySym::Asciicircum => {
                sock.send(true);
                self.symbol_svc.handle_symbol(key);
            }
            FcitxKeySym::Space
            | FcitxKeySym::Return
            | FcitxKeySym::Minus
            | FcitxKeySym::Equal
            | FcitxKeySym::Up
            | FcitxKeySym::Down
            | FcitxKeySym::Left
            | FcitxKeySym::Right
            | FcitxKeySym::Backspace
            | FcitxKeySym::Escape => {
                self.handle_control(key, sock).await;
            }
            FcitxKeySym::ShiftL | FcitxKeySym::ControlR | FcitxKeySym::AltL => {
                sock.send(false);
            }
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
            | FcitxKeySym::Z => sock.send(false),
            _ => sock.send(false),
        }
    }

    pub async fn handle_pinyin(&self, key: FcitxKeySym) {
        let c = key.to_char().expect("A-Z cannot be converted to a char.");

        self.preedit_svc.push(c);
        let preedit = self.preedit_svc.to_string();

        let candidates = self.client.query_candidates(&preedit, self.level[0]).await;

        self.candidate_svc.set_candidates(&candidates);
    }

    pub fn handle_select(&self, key: FcitxKeySym) {
        self.preedit_svc.clear();

        let i = key.to_usize().expect("Failed to conver the key to usize.");
        self.candidate_svc.select(i);
        self.candidate_svc.clear();
    }

    pub async fn handle_control(&self, key: FcitxKeySym, sock: &Server) {
        if !self.candidate_svc.in_session() {
            sock.send(false);
        }

        match key {
            FcitxKeySym::Space => {
                sock.send(true);

                self.handle_select(FcitxKeySym::Num1);
            }
            FcitxKeySym::Return => {
                sock.send(true);

                let preedit = self.preedit_svc.to_string();
                self.preedit_svc.clear();
                self.candidate_svc.clear();
                self.zmq
                    .lock()
                    .expect("handle_control: Failed to lock zmq.")
                    .commit_text(&preedit);
            }
            FcitxKeySym::Minus => {
                sock.send(true);

                self.candidate_svc.page_back();
            }
            FcitxKeySym::Equal => {
                sock.send(true);

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
            }
            FcitxKeySym::Up => sock.send(false), // For now, ingore
            FcitxKeySym::Down => sock.send(false), // For now, ignore
            FcitxKeySym::Left => sock.send(false), // For now, ignore
            FcitxKeySym::Right => sock.send(false), // For now, ignore
            FcitxKeySym::Backspace => {
                let popped = self.preedit_svc.pop();

                if popped.is_none() {
                    sock.send(false);
                    return;
                }
                sock.send(true);

                let preedit: String = self.preedit_svc.to_string();
                let candidates = self.client.query_candidates(&preedit, self.level[0]).await;
                self.candidate_svc.set_candidates(&candidates);
            }
            FcitxKeySym::Escape => {
                sock.send(true);

                self.preedit_svc.clear();
                self.candidate_svc.clear();
            }
            _ => {
                sock.send(false);
                println!("Invalid control key.")
            }
        }
    }
}
