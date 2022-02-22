use std::{io::{Write, stdout, stdin}, time::{Duration, Instant}};

use ureq::Agent;

fn main() {
    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(5))
        .build();
    
    loop {
        let mut input = String::new();
        let _ = stdout().flush();
        stdin().read_line(&mut input).expect("Did not enter a corect string.");
        
        let now = Instant::now();
        let json = get_candidate_json(&input, &agent);
        let elapsed = now.elapsed();
        
        println!("{:#?} {}", elapsed, json);
    }
}

fn get_candidate_json(pinyin: &str, agent: &Agent) -> String {
    let request = agent
        .post("https://inputtools.google.com/request")
        .set("Content-Length", "0")
        .query("text", pinyin)
        .query("itc", "zh-t-i0-pinyin")
        .query("num", "11")
        .query("ie", "utf-8");
    
    println!("Request: {:#?}", request);
    
    let result = request.call();
    
    match result {
        Ok(result) => {
            let string_result = result.into_string();
            
            match string_result {
                Ok(string_result) => string_result,
                Err(string_result) => {
                    println!("Failed to conver to string. Error: {:#?}", string_result);
                    "".to_string()
                }
            }
        },
        Err(result) => {
            println!("Request failed with {:#?}.", result);
            "".to_string()
        }
    }
}
