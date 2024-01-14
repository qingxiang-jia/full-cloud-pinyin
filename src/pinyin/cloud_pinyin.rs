use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use regex::Regex;
use reqwest::Client;
use reqwest_middleware::ClientBuilder;

use crate::common::candidate_decoder::CandidateDecoder;
use crate::common::http2::Http2;
use crate::common::path_util::abs_config_path_fcp;

use crate::common::candidate::Candidate;

use super::pinyin_decoder::PinyinDecoder;

const IM_NAME: &str = "zh-t-i0-pinyin";

pub struct CloudPinyin<D: CandidateDecoder> {
    http2: Http2,
    re: Regex,
    decoder: D,
}

impl CloudPinyin<PinyinDecoder> {
    pub fn new() -> CloudPinyin<PinyinDecoder> {
        let cache_path = abs_config_path_fcp().join("cache");
        let client = ClientBuilder::new(Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager {
                    path: cache_path.into(),
                },
                options: HttpCacheOptions::default(),
            }))
            .build();
        CloudPinyin {
            http2: Http2::new(abs_config_path_fcp()),
            re: Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input."),
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
        let mut candidates = self.json_to_candidates(json);
        for candidate in candidates.iter_mut() {
            if candidate.matched_len == 0 {
                candidate.matched_len = preedit.len();
            }
        }
        candidates
    }

    fn json_to_candidates(&self, s: String) -> Vec<Candidate> {
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
