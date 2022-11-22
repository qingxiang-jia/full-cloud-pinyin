mod ffi;

use std::{cell::Cell, path::PathBuf, sync::Mutex};

use regex::Regex;
use reqwest::header::USER_AGENT;
use serde::{Deserialize, Serialize};
use sled;
use std::fs;

type FnCommit = unsafe extern "C" fn(idx: u16);
type FnVoid = unsafe extern "C" fn();
type FnSetCandidates = unsafe extern "C" fn(candidates: *const *const i8, cnt: usize);
type FnSetPreedit = unsafe extern "C" fn(preedit: *const i8);

#[no_mangle]
pub extern "C" fn on_key_press(key: FcitxKey) {
    println!("Rust: {:#?}", key);
}

#[no_mangle]
pub extern "C" fn register_fn_commit(callback: FnCommit) {
    callback(55);
}

#[no_mangle]
pub extern "C" fn register_fn_void(callback: FnVoid) {
    callback();
}

#[no_mangle]
pub extern "C" fn register_fn_set_candidates(callback: FnSetCandidates) {

}

#[no_mangle]
pub extern "C" fn register_fn_set_preedit(callback: FnSetPreedit) {
    
}

#[repr(C)]
pub struct Fcitx5 {
    ui: UI,
    table: Table,
    engine: Engine,
}

#[repr(C)]
pub struct UI {
    set_loading: FnVoid,
    set_candidates: FnSetCandidates,
    append_candidates: FnSetCandidates,
    set_preedit: FnSetPreedit,
}

#[repr(C)]
pub struct Table {
    page_up: FnVoid,
    page_down: FnVoid,
    prev: FnVoid,
    next: FnVoid,
}

#[repr(C)]
pub struct Engine {
    commit: FnCommit,
    commit_candidate_by_fixed_key: FnVoid,
}

#[derive(Debug)]
#[repr(u32)]
pub enum FcitxKey {
    None = 0x0,
    VoidSymbol = 0xffffff, /* Void symbol */
    BackSpace = 0xff08,    /* Back space, back char */
    Tab = 0xff09,
    Return = 0xff0d, /* Return, enter */
    Pause = 0xff13,  /* Pause, hold */
    ScrollLock = 0xff14,
    Escape = 0xff1b,
    Delete = 0xffff, /* Delete, rubout */
    Home = 0xff50,
    Left = 0xff51,  /* Move left, left arrow */
    Up = 0xff52,    /* Move up, up arrow */
    Right = 0xff53, /* Move right, right arrow */
    Down = 0xff54,  /* Move down, down arrow */
    PageUp = 0xff55,
    PageDown = 0xff56,
    End = 0xff57,    /* EOL */
    Begin = 0xff58,  /* BOL */
    Select = 0xff60, /* Select, mark */
    Print = 0xff61,
    Execute = 0xff62, /* Execute, run, do */
    Insert = 0xff63,  /* Insert, insert here */
    NumLock = 0xff7f,
    F1 = 0xffbe,
    F2 = 0xffbf,
    F3 = 0xffc0,
    F4 = 0xffc1,
    F5 = 0xffc2,
    F6 = 0xffc3,
    F7 = 0xffc4,
    F8 = 0xffc5,
    F9 = 0xffc6,
    F10 = 0xffc7,
    F11 = 0xffc8,
    F12 = 0xffc9,
    ShiftL = 0xffe1,       /* Left shift */
    ShiftR = 0xffe2,       /* Right shift */
    ControlL = 0xffe3,     /* Left control */
    ControlR = 0xffe4,     /* Right control */
    CapsLock = 0xffe5,     /* Caps lock */
    ShiftLock = 0xffe6,    /* Shift lock */
    MetaL = 0xffe7,        /* Left meta */
    MetaR = 0xffe8,        /* Right meta */
    AltL = 0xffe9,         /* Left alt */
    AltR = 0xffea,         /* Right alt */
    SuperL = 0xffeb,       /* Left super */
    SuperR = 0xffec,       /* Right super */
    HyperL = 0xffed,       /* Left hyper */
    HyperR = 0xffee,       /* Right hyper */
    Space = 0x0020,        /* U+0020 SPACE */
    Exclam = 0x0021,       /* U+0021 EXCLAMATION MARK */
    QuoteDbl = 0x0022,     /* U+0022 QUOTATION MARK */
    NumberSign = 0x0023,   /* U+0023 NUMBER SIGN */
    Dollar = 0x0024,       /* U+0024 DOLLAR SIGN */
    Percent = 0x0025,      /* U+0025 PERCENT SIGN */
    Ampersand = 0x0026,    /* U+0026 AMPERSAND */
    Apostrophe = 0x0027,   /* U+0027 APOSTROPHE */
    ParenLeft = 0x0028,    /* U+0028 LEFT PARENTHESIS */
    ParenRight = 0x0029,   /* U+0029 RIGHT PARENTHESIS */
    Asterisk = 0x002a,     /* U+002A ASTERISK */
    Plus = 0x002b,         /* U+002B PLUS SIGN */
    Comma = 0x002c,        /* U+002C COMMA */
    Minus = 0x002d,        /* U+002D HYPHEN-MINUS */
    Period = 0x002e,       /* U+002E FULL STOP */
    Slash = 0x002f,        /* U+002F SOLIDUS */
    Zero = 0x0030,         /* U+0030 DIGIT ZERO */
    One = 0x0031,          /* U+0031 DIGIT ONE */
    Two = 0x0032,          /* U+0032 DIGIT TWO */
    Three = 0x0033,        /* U+0033 DIGIT THREE */
    Four = 0x0034,         /* U+0034 DIGIT FOUR */
    Five = 0x0035,         /* U+0035 DIGIT FIVE */
    Six = 0x0036,          /* U+0036 DIGIT SIX */
    Seven = 0x0037,        /* U+0037 DIGIT SEVEN */
    Eight = 0x0038,        /* U+0038 DIGIT EIGHT */
    Nine = 0x0039,         /* U+0039 DIGIT NINE */
    Colon = 0x003a,        /* U+003A COLON */
    Semicolon = 0x003b,    /* U+003B SEMICOLON */
    Less = 0x003c,         /* U+003C LESS-THAN SIGN */
    Equal = 0x003d,        /* U+003D EQUALS SIGN */
    Hreater = 0x003e,      /* U+003E GREATER-THAN SIGN */
    Question = 0x003f,     /* U+003F QUESTION MARK */
    At = 0x0040,           /* U+0040 COMMERCIAL AT */
    UpperA = 0x0041,       /* U+0041 LATIN CAPITAL LETTER A */
    UpperB = 0x0042,       /* U+0042 LATIN CAPITAL LETTER B */
    UpperC = 0x0043,       /* U+0043 LATIN CAPITAL LETTER C */
    UpperD = 0x0044,       /* U+0044 LATIN CAPITAL LETTER D */
    UpperE = 0x0045,       /* U+0045 LATIN CAPITAL LETTER E */
    UpperF = 0x0046,       /* U+0046 LATIN CAPITAL LETTER F */
    UpperG = 0x0047,       /* U+0047 LATIN CAPITAL LETTER G */
    UpperH = 0x0048,       /* U+0048 LATIN CAPITAL LETTER H */
    UpperI = 0x0049,       /* U+0049 LATIN CAPITAL LETTER I */
    UpperJ = 0x004a,       /* U+004A LATIN CAPITAL LETTER J */
    UpperK = 0x004b,       /* U+004B LATIN CAPITAL LETTER K */
    UpperL = 0x004c,       /* U+004C LATIN CAPITAL LETTER L */
    UpperM = 0x004d,       /* U+004D LATIN CAPITAL LETTER M */
    UpperN = 0x004e,       /* U+004E LATIN CAPITAL LETTER N */
    UpperO = 0x004f,       /* U+004F LATIN CAPITAL LETTER O */
    UpperP = 0x0050,       /* U+0050 LATIN CAPITAL LETTER P */
    UpperQ = 0x0051,       /* U+0051 LATIN CAPITAL LETTER Q */
    UpperR = 0x0052,       /* U+0052 LATIN CAPITAL LETTER R */
    UpperS = 0x0053,       /* U+0053 LATIN CAPITAL LETTER S */
    UpperT = 0x0054,       /* U+0054 LATIN CAPITAL LETTER T */
    UpperU = 0x0055,       /* U+0055 LATIN CAPITAL LETTER U */
    UpperV = 0x0056,       /* U+0056 LATIN CAPITAL LETTER V */
    UpperW = 0x0057,       /* U+0057 LATIN CAPITAL LETTER W */
    UpperX = 0x0058,       /* U+0058 LATIN CAPITAL LETTER X */
    UpperY = 0x0059,       /* U+0059 LATIN CAPITAL LETTER Y */
    UpperZ = 0x005a,       /* U+005A LATIN CAPITAL LETTER Z */
    BracketLeft = 0x005b,  /* U+005B LEFT SQUARE BRACKET */
    Backslash = 0x005c,    /* U+005C REVERSE SOLIDUS */
    BracketRight = 0x005d, /* U+005D RIGHT SQUARE BRACKET */
    AsciiCircum = 0x005e,  /* U+005E CIRCUMFLEX ACCENT */
    Underscore = 0x005f,   /* U+005F LOW LINE */
    Grave = 0x0060,        /* U+0060 GRAVE ACCENT */
    LowerA = 0x0061,       /* U+0061 LATIN SMALL LETTER A */
    LowerB = 0x0062,       /* U+0062 LATIN SMALL LETTER B */
    LowerC = 0x0063,       /* U+0063 LATIN SMALL LETTER C */
    LowerD = 0x0064,       /* U+0064 LATIN SMALL LETTER D */
    LowerE = 0x0065,       /* U+0065 LATIN SMALL LETTER E */
    LowerF = 0x0066,       /* U+0066 LATIN SMALL LETTER F */
    LowerG = 0x0067,       /* U+0067 LATIN SMALL LETTER G */
    LowerH = 0x0068,       /* U+0068 LATIN SMALL LETTER H */
    LowerI = 0x0069,       /* U+0069 LATIN SMALL LETTER I */
    LowerJ = 0x006a,       /* U+006A LATIN SMALL LETTER J */
    LowerK = 0x006b,       /* U+006B LATIN SMALL LETTER K */
    LowerL = 0x006c,       /* U+006C LATIN SMALL LETTER L */
    LowerM = 0x006d,       /* U+006D LATIN SMALL LETTER M */
    LowerN = 0x006e,       /* U+006E LATIN SMALL LETTER N */
    LowerO = 0x006f,       /* U+006F LATIN SMALL LETTER O */
    LowerP = 0x0070,       /* U+0070 LATIN SMALL LETTER P */
    LowerQ = 0x0071,       /* U+0071 LATIN SMALL LETTER Q */
    LowerR = 0x0072,       /* U+0072 LATIN SMALL LETTER R */
    LowerS = 0x0073,       /* U+0073 LATIN SMALL LETTER S */
    LowerT = 0x0074,       /* U+0074 LATIN SMALL LETTER T */
    LowerU = 0x0075,       /* U+0075 LATIN SMALL LETTER U */
    LowerV = 0x0076,       /* U+0076 LATIN SMALL LETTER V */
    LowerW = 0x0077,       /* U+0077 LATIN SMALL LETTER W */
    LowerX = 0x0078,       /* U+0078 LATIN SMALL LETTER X */
    LowerY = 0x0079,       /* U+0079 LATIN SMALL LETTER Y */
    LowerZ = 0x007a,       /* U+007A LATIN SMALL LETTER Z */
    BraceLeft = 0x007b,    /* U+007B LEFT CURLY BRACKET */
    Bar = 0x007c,          /* U+007C VERTICAL LINE */
    BraceRight = 0x007d,   /* U+007D RIGHT CURLY BRACKET */
    AsciiTilde = 0x007e,   /* U+007E TILDE */
    NoBreakSpace = 0x00a0, /* U+00A0 NO-BREAK SPACE */
    Exclamdown = 0x00a1,   /* U+00A1 INVERTED EXCLAMATION MARK */
    Cent = 0x00a2,         /* U+00A2 CENT SIGN */
    Sterling = 0x00a3,     /* U+00A3 POUND SIGN */
    Currency = 0x00a4,     /* U+00A4 CURRENCY SIGN */
    Yen = 0x00a5,          /* U+00A5 YEN SIGN */
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Candidates {
    depth: QueryDepth,
    candidates: Vec<Candidate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candidate {
    pub word: String,
    pub annotation: String,
    pub matched_len: Option<i32>,
}

#[derive(Debug)]
pub struct FullCloudPinyin {
    http: reqwest::blocking::Client,
    cache: sled::Db,
    last_query: Mutex<String>,
    query_depth: Cell<QueryDepth>,
    re: Regex,
}

impl FullCloudPinyin {
    pub fn new() -> Self {
        let mut path = match Self::make_config_dir_if_not_already() {
            Ok(path_buf) => path_buf,
            Err(error) => panic!("Failed to create config dir: {:#?}", error),
        };
        path.push("sled_cache");

        let config = sled::Config::default()
            .path(path.as_path())
            .cache_capacity(100 * 1024 * 1024)
            .flush_every_ms(Some(5 * 60 * 1000));

        let db = match config.open() {
            Ok(db) => db,
            Err(error) => panic!("Failed to create cache: {:#?}", error),
        };

        Self {
            http: reqwest::blocking::Client::new(),
            cache: db,
            last_query: Mutex::new("".to_owned()),
            query_depth: Cell::new(QueryDepth::D1),
            re: Regex::new("[^\"\\[\\],\\{\\}]+").expect("Invalid regex input."),
        }
    }

    pub fn query_candidates(&self, preedit: &str) -> Vec<Candidate> {
        let mut last_query = self.last_query.lock().expect("Failed to lock last_query.");
        if last_query.eq(preedit) {
            match self.query_depth.get() {
                QueryDepth::D1 => self.query_depth.set(QueryDepth::D2),
                QueryDepth::D2 => self.query_depth.set(QueryDepth::D3),
                QueryDepth::D3 => self.query_depth.set(QueryDepth::D4),
                QueryDepth::D4 => self.query_depth.set(QueryDepth::D5),
                QueryDepth::D5 => self.query_depth.set(QueryDepth::D6),
                QueryDepth::D6 => self.query_depth.set(QueryDepth::D7),
                QueryDepth::D7 => self.query_depth.set(QueryDepth::D8),
                QueryDepth::D8 => self.query_depth.set(QueryDepth::D8),
            }
        } else {
            *last_query = preedit.to_owned();
            self.query_depth.set(QueryDepth::D1);
        }
        return self.get_candidates(preedit, self.query_depth.get());
    }

    fn get_candidates(&self, preedit: &str, depth: QueryDepth) -> Vec<Candidate> {
        if preedit.len() == 0 {
            return Vec::new(); // Otherwise we will get FAILED_TO_PARSE_REQUEST_BODY
        }

        let has_key = self.cache.contains_key(preedit).expect(&format!(
            "Cache failed when trying get whether {} exists.",
            preedit
        ));
        if has_key {
            let cached = self
                .cache
                .get(preedit)
                .expect(&format!(
                    "Error occured when getting cached value for {}",
                    preedit
                ))
                .expect(&format!("The cached value for {} doesn't exist.", preedit));

            let mut deserialized: Candidates =
                bincode::deserialize(&cached).expect("The cached value cannot be deserialized.");

            if deserialized.depth > depth {
                deserialized.candidates.truncate(depth as usize);
            }

            if deserialized.depth >= depth {
                return deserialized.candidates;
            }
        }

        let url = format!("https://inputtools.google.com/request?text={}&itc=zh-t-i0-pinyin&num={}&cp=0&cs=1&ie=utf-8&oe=utf-8&app=demopage", preedit, depth as i32);

        let rep = self
            .http
            .get(url)
            .header(
                USER_AGENT,
                "Mozilla/5.0 (X11; Linux x86_64; rv:106.0) Gecko/20100101 Firefox/106.0",
            )
            .send()
            .expect("Network problems.");

        let json_str = rep.text().expect("The data cannot be converted to string.");

        let candidates = self.from_json_str_to_structured(json_str);

        // Save to cache
        let to_be_saved = Candidates {
            depth,
            candidates: candidates.clone(),
        };
        let serialized = match bincode::serialize(&to_be_saved) {
            Ok(data) => data,
            Err(error) => panic!("Failed to serialize: {:#?}", error),
        };

        _ = self.cache.insert(preedit, serialized);

        candidates
    }

    fn from_json_str_to_structured(&self, s: String) -> Vec<Candidate> {
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

    fn make_config_dir_if_not_already() -> std::io::Result<PathBuf> {
        let mut path = home::home_dir().expect("Failed to get home path.");
        path.push(".config");
        path.push("fcpinyin/");
        let result = match fs::create_dir_all(path.as_path()) {
            Ok(()) => Ok(path),
            Err(error) => Err(error),
        };
        result
    }
}
