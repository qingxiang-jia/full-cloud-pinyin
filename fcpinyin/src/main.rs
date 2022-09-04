use std::{
    fs,
    io::{stdin, stdout, Write},
    time::{Duration, Instant},
};

use regex::Regex;

#[derive(Debug)]
struct FullCloudPinyin {
    http: reqwest::blocking::Client,
}

#[derive(Debug)]
struct GitpResponse {
    // GITP stands for Google Input Tools Pinyin
    latency: Duration,
    candidates: Vec<Candidate>,
}

#[derive(Debug)]
struct CloudResponse {
    latency: Duration,
    candidates: Vec<Candidate>,
}

#[derive(Debug)]
struct Candidate {
    word: String,
    annotation: String,
    category: i8,
    lc: i32,
    matched_len: Option<i32>,
}

impl FullCloudPinyin {
    pub fn new() -> Self {
        Self {
            http: reqwest::blocking::Client::new(),
        }
    }

    pub fn get_candidates(&self, preedit: &str, depth: i32) -> CloudResponse {
        let url = format!("https://inputtools.google.com/request?text={}&itc=zh-t-i0-pinyin&num={}&cp=0&cs=1&ie=utf-8&oe=utf-8", preedit, depth);

        let now = Instant::now();

        let rep = self.http.get(url).send().expect("Network problems.");

        let elapsed = now.elapsed();

        let json_str = rep.text().expect("The data cannot be converted to string.");

        let candidates = Self::from_json_str_to_structured(json_str);

        CloudResponse {
            latency: elapsed,
            candidates,
        }
    }

    fn from_json_str_to_structured(s: String) -> Vec<Candidate> {
        let mut linear_data: Vec<String> = Vec::new();

        let re = Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input.");
        for caps in re.captures_iter(&s) {
            for cap in caps.iter() {
                if cap.is_some() {
                    linear_data.push(cap.unwrap().as_str().to_owned());
                }
            }
        }

        let mut colon_pos: Vec<usize> = Vec::new();

        assert!(linear_data[0] == "SUCCESS");

        for i in 0..linear_data.len() {
            println!("{}", &linear_data[i]);
            if linear_data[i] == ":" {
                colon_pos.push(i);
            }
        }

        let has_matched_len = colon_pos.len() == 4;

        let candidates = &linear_data[2..colon_pos[0] - 1];
        let annotations = &linear_data[colon_pos[0] + 1..colon_pos[1] - 1];
        let candidate_types = &linear_data[colon_pos[1] + 1..colon_pos[2] - 1];

        let lc: &[String];
        if has_matched_len {
            lc = &linear_data[colon_pos[2] + 1..colon_pos[3] - 1];
        } else {
            lc = &linear_data[colon_pos[2] + 1..];
        }

        let matched_len: Option<&[String]>;
        if has_matched_len {
            matched_len = Some(&linear_data[colon_pos[3] + 1..]);
        } else {
            matched_len = None;
        }

        let mut aggregate: Vec<Candidate> = Vec::new();
        for i in 0..candidates.len() {
            aggregate.push(Candidate {
                word: candidates[i].to_owned(),
                annotation: annotations[i].to_owned(),
                category: candidate_types[i]
                    .parse::<i8>()
                    .expect("Candidate type failed to be parsed to i8."),
                lc: lc[i]
                    .parse::<i32>()
                    .expect("lc faield to be parsed to i32."),
                matched_len: match matched_len {
                    Some(len) => Some(
                        len[i]
                            .parse::<i32>()
                            .expect("Matched length faield to be parsed to i32."),
                    ),
                    _ => None,
                },
            })
        }

        aggregate
    }
}

fn main() {
    let s = fs::read_to_string("src/sample.json").expect("IO error");

    let mut linear_data: Vec<String> = Vec::new();

    let re = Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input.");
    for caps in re.captures_iter(&s) {
        for cap in caps.iter() {
            if cap.is_some() {
                linear_data.push(cap.unwrap().as_str().to_owned());
            }
        }
    }

    let mut colon_pos: Vec<usize> = Vec::new();

    assert!(linear_data[0] == "SUCCESS");

    for i in 0..linear_data.len() {
        println!("{}", &linear_data[i]);
        if linear_data[i] == ":" {
            colon_pos.push(i);
        }
    }

    let has_matched_len = colon_pos.len() == 4;

    let candidates = &linear_data[2..colon_pos[0] - 1];
    let annotations = &linear_data[colon_pos[0] + 1..colon_pos[1] - 1];
    let candidate_types = &linear_data[colon_pos[1] + 1..colon_pos[2] - 1];

    let lc: &[String];
    if has_matched_len {
        lc = &linear_data[colon_pos[2] + 1..colon_pos[3] - 1];
    } else {
        lc = &linear_data[colon_pos[2] + 1..];
    }

    let matched_len: Option<&[String]>;
    if has_matched_len {
        matched_len = Some(&linear_data[colon_pos[3] + 1..]);
    } else {
        matched_len = None;
    }

    let mut aggregate: Vec<Candidate> = Vec::new();
    for i in 0..candidates.len() {
        aggregate.push(Candidate {
            word: candidates[i].to_owned(),
            annotation: annotations[i].to_owned(),
            category: candidate_types[i]
                .parse::<i8>()
                .expect("Candidate type failed to be parsed to i8."),
            lc: lc[i]
                .parse::<i32>()
                .expect("lc faield to be parsed to i32."),
            matched_len: match matched_len {
                Some(len) => Some(
                    len[i]
                        .parse::<i32>()
                        .expect("Matched length faield to be parsed to i32."),
                ),
                _ => None,
            },
        })
    }

    // Then we could just use colon positon to do list comprehensive for each part of the data
}

fn interactive_loop() {
    let client = reqwest::blocking::Client::new();
    loop {
        let mut input = String::new();
        let _ = stdout().flush();
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a corect string.");

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
    let cand_val_vec_ref = cand_val_ref
        .as_array()
        .expect("Casting failed because of unexpected JSON structure.");
    let mut candidates = Vec::new();
    for val_ref in cand_val_vec_ref.iter() {
        match val_ref {
            serde_json::Value::String(candidate) => candidates.push(candidate.clone()),
            _ => (),
        }
    }
    return candidates;
}
