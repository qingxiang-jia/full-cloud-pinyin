use regex::Regex;

use crate::common::{candidate::Candidate, candidate_decoder::CandidateDecoder};

pub struct NepaliDecoder {
    re: Regex,
}

impl CandidateDecoder for NepaliDecoder {
    fn new() -> NepaliDecoder {
        NepaliDecoder {
            re: Regex::new("todo").expect("Invalid regex input."),
        }
    }

    fn decode(&self, candidates_json: &str) -> Vec<Candidate> {
        todo!()
    }
}
