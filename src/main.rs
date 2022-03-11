use std::{io::{Write, stdout, stdin}, time::Instant};

fn main() {
    let client = reqwest::blocking::Client::new();
    loop {
        let mut input = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Did not enter a corect string.");
        
        let now = Instant::now();

        let response = get_candidates(&input, &client);
        println!("{:#?}", response);

        let elapsed = now.elapsed();
        println!("{:#?}", elapsed);
    }
}

fn get_candidates(pinyin: &str, client: &reqwest::blocking::Client) -> String {
    let data = client.get(
        format!("https://inputtools.google.com/request?text={}&itc=zh-t-i0-pinyin&num=11&cp=0&cs=1&ie=utf-8&oe=utf-8",
        pinyin.strip_suffix('\n').expect("Nothing to return after stirpping.")))
        .send()
        .expect("Network problems.")
        .text()
        .expect("The data cannot be converted to string.");
    data
}