use zbus::Connection;

use super::{ibus_proxy::IBusProxy, symbol_service::SymbolService};

pub struct NumberService {
    ibus: IBusProxy
}

impl NumberService {
    pub fn new(conn: &Connection) -> SymbolService {
        SymbolService {
            ibus: IBusProxy::new(conn),
        }
    }
}
