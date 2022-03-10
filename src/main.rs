use std::{io::{Write, stdout, stdin}, time::Instant, net::SocketAddrV4};
use std::net::{SocketAddr, ToSocketAddrs};

fn main() {
    // DNS lookup first
    let addr = get_ip().expect("No IPv4 address found.");
    
    let mut config = quiche::Config::new(quiche::PROTOCOL_VERSION).expect("Failed to initialize a configuration.");
    let scid = quiche::ConnectionId::from_vec(vec![3]);
    let conn = quiche::connect(None, &scid, SocketAddr::V4(addr), &mut config);

    // loop {
    //     let mut input = String::new();
    //     let _ = stdout().flush();
    //     stdin().read_line(&mut input).expect("Did not enter a corect string.");
        
    //     let now = Instant::now();

    //     let elapsed = now.elapsed();
    //     println!("{:#?}", elapsed);
    // }
}

fn get_ip() -> Option<SocketAddrV4> {
    let addr_iter = "inputtools.google.com:443".to_socket_addrs().expect("DNS lookup failed");
    let mut maybe_addr = None;
    addr_iter.for_each(|addr| {
        match addr {
            SocketAddr::V4(ipv4) => {
                maybe_addr = Some(ipv4);
            },
            _ => ()
        }
    });
    maybe_addr
}