mod ffi;

use std::{cell::{RefCell, Cell}, time::Instant, sync::Mutex};

use regex::Regex;
use reqwest::header::USER_AGENT;

#[derive(Debug)]
pub struct FullCloudPinyin {
    pub http: reqwest::blocking::Client,
    last_query: Mutex<String>,
    query_depth: Cell<QueryDepth>,
    re: Regex,
}

#[derive(Debug)]
pub struct Candidate {
    pub word: String,
    pub annotation: String,
    pub category: i8,
    pub lc: String,
    pub matched_len: Option<i32>,
}

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
enum QueryDepth {
    D1 = 11,
    D2 = 21,
    D3 = 41,
    D4 = 81,
    D5 = 161,
    D6 = 321,
    D7 = 641,
    D8 = 1281
}

impl FullCloudPinyin {
    pub fn new() -> Self {
        Self {
            http: reqwest::blocking::Client::new(),
            last_query: Mutex::new("".to_owned()),
            query_depth: Cell::new(QueryDepth::D1),
            re: Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input."),
        }
    }

    pub fn query_candidates(&self, preedit: &str) -> Vec<Candidate> {
        let mut last_query = self.last_query.lock().expect("Failed to lock last_query.");
        if last_query.eq(preedit) {
            match self.query_depth.get() {
                QueryDepth::D1 => self.query_depth.set(QueryDepth::D2),
                QueryDepth::D2 => self.query_depth.set(QueryDepth::D3),
                QueryDepth::D3 => self.query_depth.set(QueryDepth::D4),
                QueryDepth::D4 => self.query_depth.set(QueryDepth::D5),
                QueryDepth::D5 => self.query_depth.set(QueryDepth::D6),
                QueryDepth::D6 => self.query_depth.set(QueryDepth::D7),
                QueryDepth::D7 => self.query_depth.set(QueryDepth::D8),
                QueryDepth::D8 => self.query_depth.set(QueryDepth::D8),
            }
        } else {
            *last_query = preedit.to_owned();
            self.query_depth.set(QueryDepth::D1);
        }
        return self.get_candidates(preedit, self.query_depth.get() as i32);
    }

    fn get_candidates(&self, preedit: &str, depth: i32) -> Vec<Candidate> {
        // let start = Instant::now();
        
        if preedit.len() == 0 {
            return Vec::new(); // Otherwise we will get FAILED_TO_PARSE_REQUEST_BODY
        }

        let url = format!("https://inputtools.google.com/request?text={}&itc=zh-t-i0-pinyin&num={}&cp=0&cs=1&ie=utf-8&oe=utf-8&app=demopage", preedit, depth);

        let rep = self.http.get(url).header(USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:106.0) Gecko/20100101 Firefox/106.0").send().expect("Network problems.");

        let json_str = rep.text().expect("The data cannot be converted to string.");

        let candidates = self.from_json_str_to_structured(json_str);

        // let duration = start.elapsed().as_millis();
        // println!("{}", duration);

        candidates
    }

    fn from_json_str_to_structured(&self, s: String) -> Vec<Candidate> {
        let mut linear_data: Vec<String> = Vec::new();

        for caps in self.re.captures_iter(&s) {
            for cap in caps.iter() {
                if cap.is_some() {
                    linear_data.push(cap.unwrap().as_str().to_owned());
                }
            }
        }

        let mut colon_pos: Vec<usize> = Vec::new();

        if linear_data[0] != "SUCCESS" {
            println!("Rust: Google returned irregular data:\n{}", s.as_str());
            return Vec::new();
        }

        for i in 0..linear_data.len() {
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
                lc: lc[i].to_owned(),
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
