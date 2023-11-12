use tokio::{io, net::UnixDatagram};

#[tokio::main]
async fn main() -> io::Result<()> {
    rx("/home/lee/Downloads/fcitx_tx").await // A file will be created, cannot reuse existing one.
    // To send to this socket, with modern netcat, do: nc -uU /path/to/socket
}

async fn rx(path: &str) -> io::Result<()> {
    let rx = UnixDatagram::bind(path).expect(&format!("Failed to bind to {}.", path));

    loop {
        rx.writable()
            .await
            .expect("Socket failed to become writable.");

        let mut buf = [0; 1024];

        let sz = rx.recv(&mut buf).await.expect("Socket failed to receive.");

        println!("Recv {:?}", &buf[..sz]);
    }
}
