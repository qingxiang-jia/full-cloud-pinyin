use tokio::time::{sleep, Duration};

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cpp-rust-interop/src/cpp.h");
        fn print(s: String);
    }
}

async fn late_print() {
    sleep(Duration::from_millis(3000)).await;
    println!("2: I am good now");
}

#[tokio::main]
async fn main() {
    // ffi::print("hey hey hey".to_string());
    println!("1");
    late_print().await;
    println!("3");
}