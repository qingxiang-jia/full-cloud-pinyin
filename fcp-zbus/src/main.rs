#![feature(fmt_helpers_for_derive)]

use ibus::ibus::IBus;
mod ibus;

#[tokio::main]
async fn main() {
    let ibus = IBus::new();
    ibus.init().await;
}
