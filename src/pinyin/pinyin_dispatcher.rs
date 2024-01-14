use std::sync::{Arc, Mutex};

use crate::{
    common::candidate::Candidate,
    common::candidate_service::CandidateService,
    common::keys::FcitxKeySym,
    common::preedit_service::PreeditService,
    common::zmq::{Client, Server},
    common::{dispatcher::Dispatcher, user_dict::UserDict},
    pinyin::cloud_pinyin::CloudPinyin,
    pinyin::number_service::NumberService,
    pinyin::symbol_service::SymbolService,
};

use super::pinyin_decoder::PinyinDecoder;

struct State {
    partial_match: bool,
    pm_preedit: String,
    pm_candidate: String,
}

impl State {
    fn new() -> State {
        State {
            partial_match: false,
            pm_preedit: "".to_owned(),
            pm_candidate: "".to_owned(),
        }
    }
}

pub struct PinyinDispatcher {
    zmq: Arc<Mutex<Client>>,
    client: CloudPinyin<PinyinDecoder>,
    candidate_svc: CandidateService,
    preedit_svc: PreeditService,
    symbol_svc: SymbolService,
    number_svc: NumberService,
    user_dict: Mutex<UserDict>,
    state: Mutex<State>,
    level: Vec<usize>,
}

impl Dispatcher for PinyinDispatcher {
    fn new() -> PinyinDispatcher {
        let req: Arc<Mutex<Client>> = Arc::new(Mutex::new(Client::new("tcp://127.0.0.1:8086")));
        let dispatcher = PinyinDispatcher {
            zmq: req.clone(),
            client: CloudPinyin::new(),
            candidate_svc: CandidateService::new(req.clone()),
            preedit_svc: PreeditService::new(req.clone()),
            symbol_svc: SymbolService::new(req.clone()),
            number_svc: NumberService::new(req.clone()),
            user_dict: Mutex::new(UserDict::new()),
            state: Mutex::new(State::new()),
            level: vec![11, 21, 41, 81, 161, 321, 641, 1281],
        };
        {
            let dict = dispatcher
                .user_dict
                .lock()
                .expect("new: Failed to lock user_dict.");
            dict.load();
        }
        dispatcher
    }

    // True if key is accepted; false otherwise.
    async fn on_input(&self, key: FcitxKeySym, sock: &Server) {
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
                _ = sock.send(true);
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
                _ = sock.send(true);
                if self.candidate_svc.in_session() {
                    self.handle_select(key).await;
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
            | FcitxKeySym::ParenLeft
            | FcitxKeySym::ParenRight
            | FcitxKeySym::BracketLeft
            | FcitxKeySym::BracketRight
            | FcitxKeySym::Question
            | FcitxKeySym::Backslash
            | FcitxKeySym::Exclam
            | FcitxKeySym::Asciicircum => {
                _ = sock.send(true);
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
                _ = sock.send(false);
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
            | FcitxKeySym::Z => _ = sock.send(false),
            _ => _ = sock.send(false),
        }
    }
}

impl PinyinDispatcher {
    async fn handle_pinyin(&self, key: FcitxKeySym) {
        let c = key.to_char().expect("A-Z cannot be converted to a char.");

        self.preedit_svc.push(c);
        let preedit = self.preedit_svc.to_string();

        let mut candidates: Vec<Candidate> = Vec::new();
        let ud = self
            .user_dict
            .lock()
            .expect("handle_pinyin: Failed to lock user_dict.");
        let cand_from_ud = ud.get(&preedit);
        if cand_from_ud.is_some() {
            candidates.push(Candidate {
                word: cand_from_ud.unwrap(),
                annotation: preedit.clone(),
                matched_len: preedit.len(),
            });
        }

        let mut cand_from_cloud = self.client.query_candidates(&preedit, self.level[0]).await;
        candidates.append(&mut cand_from_cloud);

        self.candidate_svc.set_candidates(&candidates);
    }

    async fn handle_select(&self, key: FcitxKeySym) {
        let i = key.to_usize().expect("Failed to conver the key to usize.");
        let selected = self.candidate_svc.select(i);
        let old_preedit = self.preedit_svc.to_string();
        self.preedit_svc.clear();
        self.candidate_svc.clear();

        let mut state = self
            .state
            .lock()
            .expect("handle_select: Failed to lock state.");
        if old_preedit.len() > selected.matched_len {
            // We need to save the full preedit before it gets shorter and shorter with
            // subsequent partial match Similarly, we also need to save the candidate strings.
            // BEGIN: user custom word composing (to be saved to user dict).
            state
                .pm_preedit
                .push_str(&old_preedit[0..selected.matched_len]);
            state.pm_candidate.push_str(&selected.word);
            state.partial_match = true;
            drop(state);

            // It's getting the first matched_len bytes, but since we only have a-z, it's fine.
            let new_preedit = &old_preedit[selected.matched_len..];
            self.preedit_svc.push_str(new_preedit);
            let candidates = self
                .client
                .query_candidates(new_preedit, self.level[0])
                .await;
            self.candidate_svc.set_candidates(&candidates);
        } else {
            if state.partial_match {
                // END: user custom word composing (to be saved to user dict).
                let dict = self
                    .user_dict
                    .lock()
                    .expect("handle_select: Failed to lock user_dict");
                state
                    .pm_preedit
                    .push_str(&old_preedit[0..selected.matched_len]);
                state.pm_candidate.push_str(&selected.word);
                dict.insert(&state.pm_preedit, &state.pm_candidate);
                state.pm_preedit.clear();
                state.pm_candidate.clear();
            }
            state.partial_match = false;
            drop(state);
        }
    }

    async fn handle_control(&self, key: FcitxKeySym, sock: &Server) {
        if !self.candidate_svc.in_session() {
            _ = sock.send(false);
            return;
        }

        match key {
            FcitxKeySym::Space => {
                _ = sock.send(true);

                self.handle_select(FcitxKeySym::Num1).await;
            }
            FcitxKeySym::Return => {
                _ = sock.send(true);

                let preedit = self.preedit_svc.to_string();
                self.preedit_svc.clear();
                self.candidate_svc.clear();
                self.zmq
                    .lock()
                    .expect("handle_control: Failed to lock zmq.")
                    .commit_text(&preedit);
            }
            FcitxKeySym::Minus => {
                _ = sock.send(true);

                self.candidate_svc.page_back();
            }
            FcitxKeySym::Equal => {
                _ = sock.send(true);

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
            FcitxKeySym::Up => _ = sock.send(false), // For now, ingore
            FcitxKeySym::Down => _ = sock.send(false), // For now, ignore
            FcitxKeySym::Left => _ = sock.send(false), // For now, ignore
            FcitxKeySym::Right => _ = sock.send(false), // For now, ignore
            FcitxKeySym::Backspace => {
                let popped = self.preedit_svc.pop();

                if popped.is_none() {
                    _ = sock.send(false);
                    return;
                }
                _ = sock.send(true);

                let preedit: String = self.preedit_svc.to_string();
                let candidates = self.client.query_candidates(&preedit, self.level[0]).await;
                self.candidate_svc.set_candidates(&candidates);
            }
            FcitxKeySym::Escape => {
                _ = sock.send(true);

                self.preedit_svc.clear();
                self.candidate_svc.clear();
            }
            _ => {
                _ = sock.send(false);
                println!("Invalid control key.")
            }
        }
    }
}
