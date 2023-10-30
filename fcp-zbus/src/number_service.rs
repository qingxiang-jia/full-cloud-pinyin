use zbus::Connection;

use crate::keys::Key;

use super::ibus_proxy::IBusProxy;

pub struct NumberService {
    ibus: IBusProxy
}

impl NumberService {
    pub fn new(conn: &Connection) -> NumberService {
        NumberService {
            ibus: IBusProxy::new(conn),
        }
    }

    pub async fn handle_number(&self, key: Key) {
        let n = key.to_usize().expect("This key cannot be converted to a usize.");

        let text = n.to_string();
        
        self.ibus.commit_text(&text).await;
    }
}
