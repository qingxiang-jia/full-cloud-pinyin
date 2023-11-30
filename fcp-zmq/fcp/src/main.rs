use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || (args[1] != "send" && args[1] != "recv") {
        println!("Usage: parameter is either send or recv.");
        return;
    }
    let param = &args[1];
    
    if param == "send" {
    }
    if param == "recv" {
    }
}

fn run_send() {
    let mut input = String::new();
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
            }
            Err(e) => println!("Failed to read from stdin: {}", e),
        }
    }
}

fn run_recv() {
    loop {
    }
}
