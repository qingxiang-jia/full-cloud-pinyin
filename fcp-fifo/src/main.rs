use std::{
    env,
    fs::File,
    io::{self, Read, Write},
    path::Path, os::unix::fs::FileExt,
};

use nix::{sys::stat::Mode, unistd::mkfifo};

fn main() {
    let path = "/home/lee/Downloads/my_fifo";
    make_fifo(path);

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || (args[1] != "send" && args[1] != "recv") {
        println!("Usage: parameter is either send or recv.");
        return;
    }
    let param = &args[1];
    if param == "send" {
        let mut pipe = File::options()
            .write(true)
            .open(path)
            .expect("Faild to open the pipe.");
        run_send(&mut pipe);
    }
    if param == "recv" {
        let mut pipe = File::options()
            .read(true)
            .open(path)
            .expect("Faild to open the pipe.");
        run_recv(&mut pipe);
    }
}

fn make_fifo(path: &str) {
    if Path::new(path).exists() {
        return;
    }
    match mkfifo(path, Mode::S_IRWXU) {
        Ok(()) => (),
        Err(e) => {
            panic!("Failed to create fifo. {}", e);
        }
    }
}

fn run_send(pipe: &mut File) {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                send(pipe, input.as_bytes());
            }
            Err(e) => println!("Failed to read from stdin: {}", e),
        }
    }
}

fn send(pipe: &mut File, payload: &[u8]) {
    match pipe.write_all(payload) {
        Ok(()) => (),
        Err(e) => panic!("Failed to send: {e}"),
    };
}

fn run_recv(pipe: &mut File) {
    println!("run_recv");
    let mut payload: Vec<u8> = Vec::new();
    loop {
        recv(pipe, &mut payload);
    }
}

fn recv(pipe: &mut File, payload: &mut Vec<u8>) {
    match pipe.read_to_end(payload) {
        Ok(size) => println!("Read {} bytes.", size),
        Err(e) => {
            panic!("Failed to read: {}", e);
        }
    }
}
