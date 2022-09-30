mod ffi;

use regex::Regex;

#[derive(Debug)]
pub struct FullCloudPinyin {
    pub http: reqwest::blocking::Client,
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

impl FullCloudPinyin {
    pub fn new() -> Self {
        Self {
            http: reqwest::blocking::Client::new(),
            re: Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input."),
        }
    }

    pub fn get_candidates(&self, preedit: &str, depth: i32) -> Vec<Candidate> {
        let url = format!("https://inputtools.google.com/request?text={}&itc=zh-t-i0-pinyin&num={}&cp=0&cs=1&ie=utf-8&oe=utf-8", preedit, depth);

        let rep = self.http.get(url).send().expect("Network problems.");

        let json_str = rep.text().expect("The data cannot be converted to string.");

        let candidates = self.from_json_str_to_structured(json_str);

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