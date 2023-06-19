extern crate std;

use crate::{generated::IBusProxy, ibus_helper::get_ibus_address};

use ibus_variants::{IBusComponent, IBusEngineDesc};
use listeners::{FactoryListener, ServiceListener};
use zbus::{zvariant::Value, ConnectionBuilder};

mod engine;
#[allow(dead_code)]
mod generated;
mod ibus_helper;
mod ibus_variants;
mod listeners;
mod ibus_proxy;

#[tokio::main]
async fn main() {
    let ibus_engine_desc = IBusEngineDesc {
        engine_name: "full-cloud-pinyin".to_owned(),
        long_name: "Full Cloud Pinyin".to_owned(),
        description: "The Full Cloud Pinyin input method".to_owned(),
        language: "en".to_owned(),
        license: "MIT".to_owned(),
        author: "Qingxiang Jia".to_owned(),
        icon: "/usr/share/icons/breeze/emblems/24@3x/emblem-checked.svg".to_owned(),
        layout: "en".to_owned(),
        rank: 0,
        hotkeys: "".to_owned(),
        symbol: "".to_owned(),
        setup: "/usr/bin/gittupref".to_owned(),
        layout_variant: "".to_owned(),
        layout_option: "".to_owned(),
        version: "0.1".to_owned(),
        textdomain: "full-cloud-pinyin".to_owned(),
    };

    let ibus_component = IBusComponent {
        component_name: "org.freedesktop.IBus.FcPinyin".to_owned(),
        description: "".to_owned(),
        version: "".to_owned(),
        license: "".to_owned(),
        author: "".to_owned(),
        homepage: "".to_owned(),
        exec: "".to_owned(),
        textdomain: "".to_owned(),
    };

    let component_to_reg = (ibus_component).into_struct(ibus_engine_desc);

    let address = get_ibus_address().expect("Failed to get IBus address.");

    let conn = ConnectionBuilder::address(address.to_owned().as_str())
        .expect("The address didn't work.")
        .build()
        .await
        .expect("Failed to build connection to IBus.");

    let ibus = IBusProxy::new(&conn)
        .await
        .expect("Failed to create IBusProxy.");

    conn.object_server()
        .at("/org/freedesktop/IBus/Factory", FactoryListener {})
        .await
        .expect("Faild to set up server object.");

    conn.object_server()
        .at(
            "/org/freedesktop/IBus/Engine/FcPinyin",
            listeners::new_input_listener(&conn).await,
        )
        .await
        .expect("Faild to set up server object.");

    conn.object_server()
        .at("/org/freedesktop/IBus/Service", ServiceListener {})
        .await
        .expect("Faild to set up server object.");

    match ibus
        .register_component(&Value::from(component_to_reg))
        .await
    {
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
