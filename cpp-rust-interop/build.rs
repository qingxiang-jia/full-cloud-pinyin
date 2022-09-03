fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/cpp.cc")
        .compile("cpp_rust_interop");
}
