#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("cpp-rust-interop/src/cpp.h");

        fn print(s: i32);
    }
}

fn main() {
    ffi::print(10);
}