#![feature(fmt_helpers_for_derive)]

use ibus::client_factory::new_ibus_client;
mod ibus;

#[tokio::main]
async fn main() {
    let ibus = new_ibus_client();
}
