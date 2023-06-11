#![feature(fmt_helpers_for_derive)]
#![feature(prelude_import)]
#[prelude_import]
extern crate std;

use crate::{
    engine::{FcpEngine, FcpFactory, FcpService},
    generated::IBusProxy,
    ibus_helper::{gen_ibus_component, get_ibus_address},
};

use zbus::{zvariant::Value, ConnectionBuilder};

mod engine;
#[allow(dead_code)]
mod generated;
mod ibus_helper;

#[tokio::main]
async fn main() {
    let address = get_ibus_address().expect("Failed to get IBus address.");

    let factory = FcpFactory {};
    let engine = FcpEngine {};
    let service = FcpService {};

    let conn = ConnectionBuilder::address(address.to_owned().as_str())
        .expect("The address didn't work.")
        .name("org.freedesktop.IBus.FcPinyin")
        .expect("Failed to set name.")
        .serve_at("/org/freedesktop/IBus/Factory", factory)
        .expect("Faild to set up server object.")
        .serve_at("/org/freedesktop/IBus/Engine/FcPinyin", engine)
        .expect("Faild to set up server object.")
        .serve_at("/org/freedesktop/IBus/Service", service)
        .expect("Failed to set up server object.")
        .build()
        .await
        .expect("Failed to build connection to IBus.");

    let ibus = IBusProxy::new(&conn)
        .await
        .expect("Failed to create IBusProxy.");

    let component = gen_ibus_component();

    match ibus.register_component(&Value::from(component)).await {
        Ok(_) => println!("Register componnet successfully!"),
        Err(e) => println!("Failed to register component! {e}"),
    }

    match conn.request_name("org.freedesktop.IBus.FcPinyin").await {
        Ok(_) => println!("Request name is successful."),
        Err(e) => {
            println!("Request name failed because {0}", e);
        }
    }

    ibus.set_global_engine("full-cloud-pinyin")
        .await
        .expect("Failed to call set_global_engine.");

    loop {
        // do something else, wait forever or timeout here:
        // handling D-Bus messages is done in the background
        std::future::pending::<()>().await;
    }
}
