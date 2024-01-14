use http_cache_reqwest::{CACacheManager, Cache, CacheMode, HttpCache, HttpCacheOptions};
use reqwest::{header::USER_AGENT, Client};
use reqwest_middleware::ClientBuilder;

pub struct Http2 {
    client: reqwest_middleware::ClientWithMiddleware,
}

impl Http2 {
    pub fn new(cache_path: &str) -> Http2 {
        let client = ClientBuilder::new(Client::new())
            .with(Cache(HttpCache {
                mode: CacheMode::Default,
                manager: CACacheManager {
                    path: cache_path.into(),
                },
                options: HttpCacheOptions::default(),
            }))
            .build();
        Http2 { client }
    }

    pub async fn get_candidates_json(&self, preedit: &str, im: &str, depth: i32) -> String {
        let url = format!("https://inputtools.google.com/request?text={}&itc={}&num={}&cp=0&cs=1&ie=utf-8&oe=utf-8&app=demopage", preedit, im, depth);

        let resp = self
            .client
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
}
