use regex::Regex;
use reqwest::{self, header::USER_AGENT};
use tokio::sync::Mutex;
use zbus::Connection;
use zvariant::Value;

use crate::{
    generated::PanelProxy,
    ibus_variants::{IBusLookupTable, IBusText}, ibus_proxy,
};

// Implementation of org.freedesktop.IBus.Engine interface

#[allow(dead_code)]
enum KeyVal {
    A = 97,
    B = 98,
    C = 99,
    D = 100,
    E = 101,
    F = 102,
    G = 103,
    H = 104,
    I = 105,
    J = 106,
    K = 107,
    L = 108,
    M = 109,
    N = 110,
    O = 111,
    P = 112,
    Q = 113,
    R = 114,
    S = 115,
    T = 116,
    U = 117,
    V = 118,
    W = 119,
    X = 120,
    Y = 121,
    Z = 122,
    Space = 32,
    Enter = 65293,
    Minus = 45,
    Equal = 61,
    Up = 65362,
    Down = 65364,
    Left = 65361,
    Right = 65363,
    Backspace = 65288,
    Escape = 65307,
    _0 = 48,
    _1 = 49,
    _2 = 50,
    _3 = 51,
    _4 = 52,
    _5 = 53,
    _6 = 54,
    _7 = 55,
    _8 = 56,
    _9 = 57,
}

#[derive(Debug, Clone)]
pub struct Candidate {
    pub word: String,
    annotation: String,
    matched_len: Option<i32>,
}

struct State {
    preedit: String,
    depth: usize,
    session: bool,
    candidates: Vec<Candidate>,
    page: usize,
}

unsafe impl Sync for State {} // State is safe to share between threads

impl State {
    pub fn new() -> Self {
        State {
            preedit: "".to_owned(),
            depth: 0,
            session: false,
            candidates: Vec::new(),
            page: 0,
        }
    }
}

pub struct FcpEngine {
    conn: Connection,
    http: reqwest::Client,
    re: Regex,
    lt_size: usize,
    levels: Vec<usize>,
    state: Mutex<State>,
}

impl FcpEngine {
    pub fn new(conn: &Connection) -> FcpEngine {
        FcpEngine {
            conn: conn.clone(),
            http: reqwest::Client::new(),
            re: Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input."),
            levels: vec![11, 21, 41, 81, 161, 321, 641, 1281],
            lt_size: 5,
            state: Mutex::new(State::new()),
        }
    }

    pub async fn on_key_press(&self, keyval: u32) -> bool {
        ibus_proxy::commit_text(&self.conn, &Value::from(IBusText::new("asadasdasdsadasdsad").into_struct())).await;
        return true;
    }

    async fn query_candidates(&self, preedit: &str, depth: usize) -> Vec<Candidate> {
        let json = self.get_candidates_from_net(preedit, depth as i32).await;
        let candidates = self.json_to_candidates(json);
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
                    Some(len) => Some(
                        len[i]
                            .parse::<i32>()
                            .expect("Matched length faield to be parsed to i32."),
                    ),
                    _ => None,
                },
            })
        }

        aggregate
    }

    fn concate(s: &String, c: u32) -> String {
        let mut new = s.clone();
        new.push(char::from_u32(c).expect(&format!("Cannot convert u32 {c} to char.")));
        return new;
    }
}
