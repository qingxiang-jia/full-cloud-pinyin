use std::sync::{Arc, Mutex};

use crate::common::{
    candidate::Candidate,
    candidate_service::CandidateService,
    dispatcher::Dispatcher,
    keys::FcitxKeySym,
    preedit_service::PreeditService,
    zmq::{Client, Server},
};

use super::{
    cloud_nepali::CloudNepali, nepali_decoder::NepaliDecoder, number_service::NumberService,
};

pub struct NepaliDispatcher {
    zmq: Arc<Mutex<Client>>,
    nepali: CloudNepali<NepaliDecoder>,
    candidate_svc: CandidateService,
    preedit_svc: PreeditService,
    number_svc: NumberService,
    level: Vec<usize>,
}

impl Dispatcher for NepaliDispatcher {
    fn new() -> NepaliDispatcher {
        let req: Arc<Mutex<Client>> = Arc::new(Mutex::new(Client::new("tcp://127.0.0.1:8086")));
        let dispatcher = NepaliDispatcher {
            zmq: req.clone(),
            nepali: CloudNepali::new(),
            candidate_svc: CandidateService::new(req.clone()),
            preedit_svc: PreeditService::new(req.clone()),
            number_svc: NumberService::new(req.clone()),
            level: vec![0],
        };
        dispatcher
    }

    async fn on_input(&self, key: FcitxKeySym, sock: &Server) {
        match key {
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
                self.handle_nepali(key).await;
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
            _ => _ = sock.send(false),
        }
    }
}

impl NepaliDispatcher {
    async fn handle_nepali(&self, key: FcitxKeySym) {
        let c = key.to_char().expect("A-Z cannot be converted to a char.");

        self.preedit_svc.push(c);
        let preedit = self.preedit_svc.to_string();

        let candidates: Vec<Candidate> =
            self.nepali.query_candidates(&preedit, self.level[0]).await;

        self.candidate_svc.set_candidates(&candidates);
    }

    async fn handle_select(&self, key: FcitxKeySym) {
        let i = key.to_usize().expect("Failed to conver the key to usize.");
        let _ = self.candidate_svc.select(i);
        self.preedit_svc.clear();
        self.candidate_svc.clear();
    }

    async fn handle_control(&self, key: FcitxKeySym, sock: &Server) {
        todo!()
    }
}
