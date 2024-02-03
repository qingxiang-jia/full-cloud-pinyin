use regex::Regex;

use crate::common::{candidate::Candidate, candidate_decoder::CandidateDecoder};

pub struct NepaliDecoder {
    re: Regex,
}

impl CandidateDecoder for NepaliDecoder {
    fn new() -> NepaliDecoder {
        NepaliDecoder {
            re: Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input."),
        }
    }

    // ["SUCCESS",[["aj",["आज","ाज","अज","एज","ाज़","जज","ेज","अज़","आज़","ऐज","ऎज","यज","अज्ज"],[],{"candidate_type":[0,0,0,0,0,0,0,0,0,0,0,0,0]}]]]

    fn decode(&self, candidates_json: &str) -> Vec<Candidate> {
        let mut linear_data: Vec<String> = Vec::new();

        for caps in self.re.captures_iter(candidates_json) {
            for cap in caps.iter() {
                if cap.is_some() {
                    linear_data.push(cap.unwrap().as_str().to_owned());
                }
            }
        }

        if linear_data[0] != "SUCCESS" {
            println!("Rust: Google returned irregular data:\n{}", candidates_json);
            return Vec::new();
        }

        let mut candidates: Vec<Candidate> = Vec::new();
        for (i, str) in linear_data.iter().enumerate() {
            if i == 0 {
                continue; // Skip status code.
            }
            if str == "candidate_type" {
                break;
            }
            candidates.push(Candidate {
                word: str.to_owned(),
                annotation: "".to_owned(),
                matched_len: 0,
            })
        }

        candidates
    }
}

#[cfg(test)]
mod tests {
    use crate::common::candidate_decoder::CandidateDecoder;

    use super::NepaliDecoder;

    #[test]
    fn test_decode() {
        let decoder = NepaliDecoder::new();
        let candidates_json = "[\"SUCCESS\",[[\"aj\",[\"आज\",\"ाज\",\"अज\",\"एज\",\"ाज़\",\"जज\",\"ेज\",\"अज़\",\"आज़\",\"ऐज\",\"ऎज\",\"यज\",\"अज्ज\"],[],{\"candidate_type\":[0,0,0,0,0,0,0,0,0,0,0,0,0]}]]]";
        let candidates = decoder.decode(candidates_json);

        for candidate in &candidates {
            assert!(candidate.word.len() > 0);
            assert!(candidate.annotation.len() == 0);
            assert!(candidate.matched_len == 0);
        }
    }
}
