use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use regex::Regex;
use reqwest::{header::USER_AGENT, Client};
use reqwest_middleware::ClientBuilder;

use crate::common::path_util::abs_config_path_fcp;

use crate::common::candidate::Candidate;

pub struct CloudPinyinClient {
    http: reqwest_middleware::ClientWithMiddleware,
    re: Regex,
}

impl CloudPinyinClient {
    pub fn new() -> CloudPinyinClient {
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
        CloudPinyinClient {
            http: client,
            re: Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input."),
        }
    }

    pub async fn query_candidates(&self, preedit: &str, depth: usize) -> Vec<Candidate> {
        if preedit.len() == 0 {
            return Vec::new();
        }
        let json = self.get_candidates_from_net(preedit, depth as i32).await;
        let mut candidates = self.json_to_candidates(json);
        for candidate in candidates.iter_mut() {
            if candidate.matched_len == 0 {
                candidate.matched_len = preedit.len();
            }
        }
        candidates
    }

    async fn get_candidates_from_net(&self, preedit: &str, depth: i32) -> String {
        let url = format!("https://inputtools.google.com/request?text={}&itc=zh-t-i0-pinyin&num={}&cp=0&cs=1&ie=utf-8&oe=utf-8&app=demopage", preedit, depth);

        let resp = self
            .http
            .get(url)
            .header(
                USER_AGENT,
                "Mozilla/5.0 (X11; Linux x86_64; rv:106.0) Gecko/20100101 Firefox/106.0",
            )
            .send()
            .await
            .expect("Network problems when making the request.");

        resp.text()
            .await
            .expect("Network problem when getting the text from the response.")
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
