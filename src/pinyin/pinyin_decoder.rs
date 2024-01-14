use regex::Regex;

use crate::common::candidate::Candidate;
use crate::common::candidate_decoder::CandidateDecoder;

pub struct PinyinDecoder {
    re: Regex,
}

impl CandidateDecoder for PinyinDecoder {
    fn new() -> PinyinDecoder {
        PinyinDecoder {
            re: Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input."),
        }
    }

    fn decode(&self, candidates_json: &str) -> Vec<Candidate> {
        let mut linear_data: Vec<String> = Vec::new();

        for caps in self.re.captures_iter(candidates_json) {
            for cap in caps.iter() {
                if cap.is_some() {
                    linear_data.push(cap.unwrap().as_str().to_owned());
                }
            }
        }

        let mut colon_pos: Vec<usize> = Vec::new();

        if linear_data[0] != "SUCCESS" {
            println!("Rust: Google returned irregular data:\n{}", candidates_json);
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
                matched_len: match matched_len {
                    Some(len) => len[i]
                        .parse::<usize>()
                        .expect("Matched length faield to be parsed to usize."),
                    _ => 0, // All candidates match the whole preedit.
                },
            })
        }

        aggregate
    }
}
