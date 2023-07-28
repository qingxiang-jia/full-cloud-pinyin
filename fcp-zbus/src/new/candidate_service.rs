use crate::ibus_proxy::IBusProxy;

pub struct CandidateService {
    lt_size: usize,
    levels: Vec<usize>,
    ibus: IBusProxy,
}