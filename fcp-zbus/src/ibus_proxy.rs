use zbus::Connection;

static ENGINE_PATH: &str = "/org/freedesktop/IBus/Engine/FcPinyin";
static ENGINE_IFACE: &str = "org.freedesktop.IBus.Engine";

pub async fn commit_text(conn: &Connection, text: &zbus::zvariant::Value<'_>) {
    conn.emit_signal(
        None::<&str>,
        ENGINE_PATH,
        ENGINE_IFACE,
        "CommitText",
        text,
    )
    .await
    .expect("Failed to emit CommitText signal.");
}

// UpdatePreeditText: text: IBusText, cursor_pos u32, visible bool, 0

// UpdateLookupTable: lt: IBusLookupTable, visible bool

// ShowLookupTable

// HideLookupTable