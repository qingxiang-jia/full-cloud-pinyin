use std::fs;

fn main() {
    let code = fs::read_to_string("ffi.cc").expect("Failed to load the file.");

    println!("{}", code);
}
