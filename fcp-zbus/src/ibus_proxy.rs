use zbus::Connection;
use zvariant::Value;

use super::ibus_variants::{IBusLookupTable, IBusText};

static ENGINE_PATH: &str = "/org/freedesktop/IBus/Engine/FcPinyin";
static ENGINE_IFACE: &str = "org.freedesktop.IBus.Engine";

pub struct IBusProxy {
    conn: Connection,
}

impl IBusProxy {
    pub fn new(conn_ref: &Connection) -> IBusProxy {
        IBusProxy {
            conn: conn_ref.clone(),
        }
    }

    pub async fn commit_text(&self, text: &str) {
        commit_text(&self.conn, &Value::from(IBusText::from_str_ref(text).into_struct())).await;
    }

    pub async fn update_preedit_text(&self, text: &str, cursor_pos: u32, visible: bool) {
        update_preedit_text(
            &self.conn,
            &Value::from(IBusText::from_str_ref(text).into_struct()),
            cursor_pos,
            visible,
        )
        .await;
    }

    pub async fn update_lookup_table(&self, lt: IBusLookupTable, visible: bool) {
        update_lookup_table(&self.conn, &Value::from(lt.into_struct()), visible).await;
    }

    pub async fn show_lookup_table(&self) {
        show_lookup_table(&self.conn).await;
    }

    pub async fn hide_lookup_table(&self) {
        hide_lookup_table(&self.conn).await;
    }
}

async fn commit_text(conn: &Connection, text: &Value<'_>) {
    conn.emit_signal(None::<&str>, ENGINE_PATH, ENGINE_IFACE, "CommitText", text)
        .await
        .expect("Failed to emit CommitText signal.");
}

async fn update_preedit_text(conn: &Connection, text: &Value<'_>, cursor_pos: u32, visible: bool) {
    conn.emit_signal(
        None::<&str>,
        ENGINE_PATH,
        ENGINE_IFACE,
        "UpdatePreeditText",
        &(text, cursor_pos, visible, 0 as u32),
    )
    .await
    .expect("Failed to emit UpdatePreeditText signal.");
}

async fn update_lookup_table(conn: &Connection, lt: &Value<'_>, visible: bool) {
    conn.emit_signal(
        None::<&str>,
        ENGINE_PATH,
        ENGINE_IFACE,
        "UpdateLookupTable",
        &(lt, visible),
    )
    .await
    .expect("Failed to emit UpdateLookupTable signal.");
}

async fn show_lookup_table(conn: &Connection) {
    conn.emit_signal(
        None::<&str>,
        ENGINE_PATH,
        ENGINE_IFACE,
        "ShowLookupTable",
        &(),
    )
    .await
    .expect("Failed to emit ShowLookupTable signal.");
}

async fn hide_lookup_table(conn: &Connection) {
    conn.emit_signal(
        None::<&str>,
        ENGINE_PATH,
        ENGINE_IFACE,
        "HideLookupTable",
        &(),
    )
    .await
    .expect("Failed to emit HideLookupTable signal.");
}
