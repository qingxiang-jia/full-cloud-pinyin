use zbus::Connection;

pub async fn commit_text(conn: &Connection, text: &zbus::zvariant::Value<'_>) {
    conn.emit_signal(
        Some("org.freedesktop.IBus.Panel"),
        "/org/freedesktop/IBus/Panel",
        "org.freedesktop.IBus.Panel",
        "CommitText",
        text,
    )
    .await
    .expect("Failed to emit CommitText signal.");
}
