#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cpp-rust-interop/src/cpp.h");
        fn print(s: String);
    }
}

fn main() {
    ffi::print("hey hey hey".to_string());
}