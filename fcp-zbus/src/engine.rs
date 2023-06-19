use regex::Regex;
use reqwest::{self, header::USER_AGENT};
use tokio::sync::Mutex;
use zvariant::Value;

use crate::{
    generated::{IBusProxy, PanelProxy},
    ibus_variants::{IBusLookupTable, IBusText},
};

// Implementation of org.freedesktop.IBus.Engine interface

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
enum QueryDepth {
    D1 = 11,
    D2 = 21,
    D3 = 41,
    D4 = 81,
    D5 = 161,
    D6 = 321,
    D7 = 641,
    D8 = 1281,
}

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
    preedit: Mutex<String>,
    depth: Mutex<QueryDepth>,
    in_session: Mutex<bool>,
    candidates: Mutex<Vec<Candidate>>,
    table_size: usize,
    page: Mutex<usize>,
    cursor: Mutex<usize>,
}

unsafe impl Sync for State {} // State is safe to share between threads

impl State {
    pub fn new() -> Self {
        State {
            preedit: Mutex::new("".to_owned()),
            depth: Mutex::new(QueryDepth::D1),
            in_session: Mutex::new(false),
            candidates: Mutex::new(Vec::new()),
            table_size: 5,
            page: Mutex::new(0),
            cursor: Mutex::new(0),
        }
    }

    pub async fn preedit(&self) -> String {
        self.preedit.lock().await.clone()
    }

    pub async fn set_preedt_atomic(&self, query: &str) {
        let mut shared = self.preedit.lock().await;
        shared.replace_range(.., query);
    }

    pub async fn candidate_cnt(&self) -> usize {
        self.candidates.lock().await.len()
    }

    pub async fn set_candidates_atomic(&self, new: &Vec<Candidate>) {
        let mut shared = self.candidates.lock().await;
        shared.clear();
        for cand in new {
            shared.push(cand.clone());
        }
    }

    pub async fn page(&self) -> usize {
        self.page.lock().await.clone()
    }

    pub async fn set_page_atomic(&self, new: usize) {
        let mut shared = self.page.lock().await;
        *shared = new;
    }

    pub async fn cursor(&self) -> usize {
        self.cursor.lock().await.clone()
    }

    pub async fn set_cursor_atomic(&self, new: usize) {
        let mut shared = self.cursor.lock().await;
        *shared = new;
    }

    pub async fn reset(&self) {
        let mut preedit = self.preedit.lock().await;
        let mut depth = self.depth.lock().await;
        let mut in_session = self.in_session.lock().await;
        let mut candidates = self.candidates.lock().await;
        let mut page = self.page.lock().await;
        let mut cursor = self.cursor.lock().await;

        *preedit = "".to_owned();
        *depth = QueryDepth::D1;
        *in_session = false;
        candidates.clear();
        *page = 0;
        *cursor = 0;
    }
}

pub struct FcpEngine<'a> {
    panel: PanelProxy<'a>,
    http: reqwest::Client,
    re: Regex,
    state: State,
}

impl<'a> FcpEngine<'a> {
    pub fn new(ibus: IBusProxy<'a>, panel: PanelProxy<'a>) -> FcpEngine<'a> {
        FcpEngine {
            panel,
            http: reqwest::Client::new(),
            re: Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input."),
            state: State::new(),
        }
    }

    pub async fn on_key_press(&self, keyval: u32) -> bool {

        return false;
    }

    async fn update_lookup_table(&self, new: IBusLookupTable, visible: bool) {
        self.panel
            .update_lookup_table(&Value::new(new.into_struct()), visible)
            .await
            .expect("Failed to update lookup table.");
    }

    async fn update_preedit(&self, new: &str) {
        self.panel
            .update_preedit_text(
                &Value::from(IBusText::new(new).into_struct()),
                0,
                new.len() != 0,
            )
            .await
            .expect("Failed to update preedit.");
    }

    async fn query_candidates(&self, preedit: &str) -> Vec<Candidate> {
        let depth = self.decide_n_update_depth(preedit).await;
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

    async fn decide_n_update_depth(&self, preedit: &str) -> QueryDepth {
        let mut last_query = self.state.preedit.lock().await;
        let mut depth = self.state.depth.lock().await;
        if last_query.eq(preedit) {
            match *depth {
                QueryDepth::D1 => *depth = QueryDepth::D2,
                QueryDepth::D2 => *depth = QueryDepth::D3,
                QueryDepth::D3 => *depth = QueryDepth::D4,
                QueryDepth::D4 => *depth = QueryDepth::D5,
                QueryDepth::D5 => *depth = QueryDepth::D6,
                QueryDepth::D6 => *depth = QueryDepth::D7,
                QueryDepth::D7 => *depth = QueryDepth::D8,
                QueryDepth::D8 => *depth = QueryDepth::D8,
            }
        } else {
            *last_query = preedit.to_owned();
            *depth = QueryDepth::D1;
        }
        depth.clone()
    }

    fn concate(s: &String, c: u32) -> String {
        let mut new = s.clone();
        new.push(char::from_u32(c).expect(&format!("Cannot convert u32 {c} to char.")));
        return new;
    }
}
