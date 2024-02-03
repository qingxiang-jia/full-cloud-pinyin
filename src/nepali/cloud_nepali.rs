use crate::common::{
    candidate::Candidate, candidate_decoder::CandidateDecoder, http2::Http2,
    path_util::abs_config_path_fcn,
};

use super::nepali_decoder::NepaliDecoder;

const IM_NAME: &str = "ne-t-i0-und";

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
        if preedit.len() == 0 {
            return Vec::new();
        }
        let json = self
            .http2
            .get_candidates_json(preedit, IM_NAME, depth as i32)
            .await;
        self.decoder.decode(&json)
    }
}
