use std::{io::{Write, stdout, stdin}, time::Instant, str::FromStr};
use hyper_tls::HttpsConnector;
use hyper::{Client, Uri, client::{HttpConnector, ResponseFuture}};

#[tokio::main]
async fn main() {
    let https = HttpsConnector::new();
    let client = Client::builder()
        .build::<_, hyper::Body>(https);

    loop {
        let mut input = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Did not enter a corect string.");
        
        let now = Instant::now();

        let future = get_candidates(&input, &client);
        let result = future.await;
        match result {
            Ok(response) => {
                let data = hyper::body::to_bytes(response.into_body()).await;
                
                match data {
                    Ok(buf) => {
                        let maybe_str = std::str::from_utf8(&buf);
                        
                        match maybe_str {
                            Ok(str) => println!("{:#?}", str),
                            Err(err) => println!("Failed to convert bytes to str: {:#?}", err)
                        }
                    },
                    Err(err) => println!("Reading body failed: {:#?}", err)
                }
            },
            Err(err) => println!("{:#?}", err)
        }

        let elapsed = now.elapsed();
        println!("{:#?}", elapsed);
    }
}

fn get_candidates(pinyin: &str, client: &Client<hyper_tls::HttpsConnector<HttpConnector>, hyper::Body>) -> ResponseFuture {
    let url = format!("https://inputtools.google.com/request?text={}&itc=zh-t-i0-pinyin&num=11&cp=0&cs=1&ie=utf-8&oe=utf-8", pinyin.strip_suffix('\n').unwrap());
    println!("request URL: {}", url);
    let future = client.get(Uri::from_str(&url).expect("Invalid URI"));
    return future;
}
