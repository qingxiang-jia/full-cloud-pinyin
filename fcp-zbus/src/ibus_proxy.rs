use zbus::Connection;
use zvariant::Value;

static ENGINE_PATH: &str = "/org/freedesktop/IBus/Engine/FcPinyin";
static ENGINE_IFACE: &str = "org.freedesktop.IBus.Engine";

pub async fn commit_text(conn: &Connection, text: &Value<'_>) {
    conn.emit_signal(None::<&str>, ENGINE_PATH, ENGINE_IFACE, "CommitText", text)
        .await
        .expect("Failed to emit CommitText signal.");
}

pub async fn update_preedit_text(
    conn: &Connection,
    text: &Value<'_>,
    cursor_pos: u32,
    visible: bool,
) {
    conn.emit_signal(
        None::<&str>,
        ENGINE_PATH,
        ENGINE_IFACE,
        "UpdatePreeditText",
        &(text, cursor_pos, visible, 0),
    )
    .await
    .expect("Failed to emit UpdatePreeditText signal.");
}

pub async fn update_lookup_table(conn: &Connection, lt: &Value<'_>, visible: bool) {
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

pub async fn show_lookup_table(conn: &Connection) {
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

pub async fn hide_lookup_table(conn: &Connection) {
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