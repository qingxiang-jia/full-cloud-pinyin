use super::candidate::Candidate;

pub trait CandidateDecoder {
    fn new() -> Self;
    fn decode(&self, candidates_json: &str) -> Vec<Candidate>;
}
