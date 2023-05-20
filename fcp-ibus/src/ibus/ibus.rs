use zbus::AuthMechanism;
use zbus::Connection;
use zbus::ConnectionBuilder;

use super::proxy::ibus::IBusProxy;

// DBus interfaces
pub struct IBus {}

impl IBus {
    pub fn new() -> Self {
        IBus {}
    }

    pub async fn init(&self) {
        // client init
        let conn_to_ibus = ConnectionBuilder::session()
            .expect("Failed to get a connection to the session bus.")
            .auth_mechanisms(&[AuthMechanism::External])
            .build()
            .await
            .expect("Failed to build a DBus connection.");

        let proxy_ibus = IBusProxy::new(&conn_to_ibus)
            .await
            .expect("Failed to get a connection to IBus.");
        proxy_ibus.exit(false).await.expect("Failed to exit.");
        // server object init
    }
}
