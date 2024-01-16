use std::sync::{Arc, Mutex};

use crate::common::{
    candidate_service::CandidateService,
    dispatcher::Dispatcher,
    keys::FcitxKeySym,
    preedit_service::PreeditService,
    zmq::{Client, Server},
};

use super::{cloud_nepali::CloudNepali, nepali_decoder::NepaliDecoder};

pub struct NepaliDispatcher {
    zmq: Arc<Mutex<Client>>,
    nepali: CloudNepali<NepaliDecoder>,
    candidate_svc: CandidateService,
    preedit_svc: PreeditService,
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
            level: vec![0],
        };
        dispatcher
    }

    async fn on_input(&self, key: FcitxKeySym, sock: &Server) {
        todo!()
    }
}
