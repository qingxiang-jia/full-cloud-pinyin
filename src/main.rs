use std::{io::{Write, stdout, stdin}, time::Instant};

fn main() {
    let client = reqwest::blocking::Client::new();
    loop {
        let mut input = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Did not enter a corect string.");
        
        let now = Instant::now();

        let response = get_candidate_json(&input, &client);

        let elapsed = now.elapsed();
        println!("{:#?}", elapsed);

        println!("{:#?}", response);
    }
}

fn get_candidate_json(pinyin: &str, client: &reqwest::blocking::Client) -> Vec<String> {
    let json: serde_json::Value = client.get(
        format!("https://inputtools.google.com/request?text={}&itc=zh-t-i0-pinyin&num=11&cp=0&cs=1&ie=utf-8&oe=utf-8",
        pinyin.strip_suffix('\n').expect("Nothing to return after stirpping.")))
        .send()
        .expect("Network problems.")
        .json()
        .expect("The data cannot be converted to JSON.");
    let cand_val_ref = &json[1][0][1];
    let cand_val_vec_ref = cand_val_ref.as_array().expect("Casting failed because of unexpected JSON structure.");
    let mut candidates = Vec::new();
    for val_ref in cand_val_vec_ref.iter() {
        match val_ref {
            serde_json::Value::String(candidate) => candidates.push(candidate.clone()),
            _ => ()
        }
    }
    return candidates;
}

//https://michael-f-bryan.github.io/rust-ffi-guide/overview.html
//http://jakegoulding.com/rust-ffi-omnibus/string_return/
//https://stackoverflow.com/questions/40156545/how-do-i-return-an-vector-of-dynamic-length-in-a-pub-extern-c-fn