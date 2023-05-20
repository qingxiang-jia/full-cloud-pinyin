use zbus::Connection;

// DBus addresses
static WELL_KNOWN_IBUS_ADDRESS: &str = "/org/freedesktop/IBus";
static WELL_KNOWN_IBUS_FACTORY_ADDRESS: &str = "/org/freedesktop/IBus/Factory";
static WELL_KNOWN_DBUS_ADDRESS: &str = "/org/freedesktop/DBus";

// DBus interfaces
static IBUS_INTERFACE: &str = "org.freedesktop.IBus";
static IBUS_FACTORY_INTERFACE: &str = "org.freedesktop.IBus.Factory";
static IBUS_ENGINE_INTERFACE: &str = "org.freedesktop.IBus.Engine";
static IBUS_SERVICE_INTERFACE: &str = "org.freedesktop.IBus.Service";
static IBUS_PROPERTIES_INTERFACE: &str = "org.freedesktop.DBus.Properties";
static DBUS_INTERFACE: &str = "org.freedesktop.DBus";

pub struct IBus {
    ibus_address: String,
    dbus_address: String,
    ibus_factory_address: String,
    ibus_interface: String,
    dbus_interface: String,
    ibus_engine_interface: String,
    ibus_service_interface: String,
    ibus_properties_interface: String,
}

impl IBus {
    pub fn new() -> Self {
        IBus {
            ibus_address: WELL_KNOWN_IBUS_ADDRESS.to_string(),
            dbus_address: WELL_KNOWN_DBUS_ADDRESS.to_string(),

            ibus_factory_address: WELL_KNOWN_IBUS_FACTORY_ADDRESS.to_string(),

            ibus_interface: IBUS_INTERFACE.to_string(),
            dbus_interface: DBUS_INTERFACE.to_string(),

            ibus_engine_interface: IBUS_ENGINE_INTERFACE.to_string(),
            ibus_service_interface: IBUS_SERVICE_INTERFACE.to_string(),
            ibus_properties_interface: IBUS_PROPERTIES_INTERFACE.to_string(),
        }
    }

    pub async fn init() {
        // client init
        let conn_to_ibus = Connection::session().await.expect("Failed to get a DBus session connection.");
        // server object init
    }
}
