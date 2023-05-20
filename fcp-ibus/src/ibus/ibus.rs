use zbus::Connection;

use super::proxy::ibus::IBusProxy;

// DBus interfaces
pub struct IBus {}

impl IBus {
    pub fn new() -> Self {
        IBus {}
    }

    pub async fn init(&self) {
        // client init
        let conn_to_ibus = Connection::session()
            .await
            .expect("Failed to get a DBus session connection.");
        let proxy_ibus = IBusProxy::new(&conn_to_ibus)
            .await
            .expect("Failed to get a connection to IBus.");
        let engines = proxy_ibus
            .engines()
            .await
            .expect("Failed to get IBus engines.");
        println!("Number of engines on IBus: {}", engines.len());
        // server object init
    }
}
