use crate::common::candidate_decoder::CandidateDecoder;
use crate::common::http2::Http2;
use crate::common::path_util::abs_config_path_fcp;

use crate::common::candidate::Candidate;

use super::pinyin_decoder::PinyinDecoder;

const IM_NAME: &str = "zh-t-i0-pinyin";

pub struct CloudPinyin<D: CandidateDecoder> {
    http2: Http2,
    decoder: D,
}

impl CloudPinyin<PinyinDecoder> {
    pub fn new() -> CloudPinyin<PinyinDecoder> {
        CloudPinyin {
            http2: Http2::new(abs_config_path_fcp()),
            decoder: PinyinDecoder::new(),
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
        let mut candidates = self.decoder.decode(&json);
        for candidate in candidates.iter_mut() {
            if candidate.matched_len == 0 {
                candidate.matched_len = preedit.len();
            }
        }
        candidates
    }
}
