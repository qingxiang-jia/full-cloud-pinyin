use crate::common::{
    candidate::Candidate, candidate_decoder::CandidateDecoder, http2::Http2,
    path_util::abs_config_path_fcn,
};

use super::nepali_decoder::NepaliDecoder;

pub struct CloudNepali<D: CandidateDecoder> {
    http2: Http2,
    decoder: D,
}

impl CloudNepali<NepaliDecoder> {
    pub fn new() -> CloudNepali<NepaliDecoder> {
        CloudNepali {
            http2: Http2::new(abs_config_path_fcn()),
            decoder: NepaliDecoder::new(),
        }
    }

    pub async fn query_candidates(&self, preedit: &str, depth: usize) -> Vec<Candidate> {
        todo!()
    }
}
