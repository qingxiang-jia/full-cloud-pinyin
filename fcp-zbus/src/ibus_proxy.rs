use zbus::Connection;

pub async fn commit_text(conn: &Connection, text: &zbus::zvariant::Value<'_>) {
    conn.emit_signal(
        None::<&str>,
        "/org/freedesktop/IBus/Engine/FcPinyin",
        "org.freedesktop.IBus.Engine",
        "CommitText",
        text,
    )
    .await
    .expect("Failed to emit CommitText signal.");
}
