use zbus::Connection;

use crate::keys::Key;

use super::ibus_proxy::IBusProxy;

pub struct SymbolService {
    pub(crate) ibus: IBusProxy
}

impl SymbolService {
    pub fn new(conn: &Connection) -> SymbolService {
        SymbolService {
            ibus: IBusProxy::new(conn),
        }
    }

    pub async fn handle_symbol(&self, key: Key) {
        let fw_puctuation = key
            .to_full_width_string()
            .expect("This key cannot be converted to fullwidth string.");

        self.ibus.commit_text(&fw_puctuation).await;
    }
}

