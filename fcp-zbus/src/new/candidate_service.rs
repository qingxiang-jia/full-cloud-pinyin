use zbus::Connection;

use crate::ibus_proxy::IBusProxy;

pub struct CandidateService {
    lt_size: usize,
    levels: Vec<usize>,
    ibus: IBusProxy,
}

impl CandidateService {
    pub fn new(conn: &Connection) -> CandidateService {
        CandidateService {
            lt_size: 5,
            levels: vec![11, 21, 41, 81, 161, 321, 641, 1281],
            ibus: IBusProxy::new(&conn),
        }
    }

    pub fn page_up() {}

    pub fn page_down() {}

    pub fn set_candidates() {}

    pub fn select() {}

    pub fn clear() {}
}
