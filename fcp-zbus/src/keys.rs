#[allow(dead_code)]
pub enum KeyVal {
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
    Comma = 44,
    Period = 46,
    SemiColon = 59,
    SingleQuote = 39,
    ForwardSlash = 47,
    Space = 32,
    Enter = 65293,
    Shift = 65505,
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

impl KeyVal {
    pub fn from_u32(num: u32) -> Option<KeyVal> {
        match num {
            97 => Some(KeyVal::A),
            98 => Some(KeyVal::B),
            99 => Some(KeyVal::C),
            100 => Some(KeyVal::D),
            101 => Some(KeyVal::E),
            102 => Some(KeyVal::F),
            103 => Some(KeyVal::G),
            104 => Some(KeyVal::H),
            105 => Some(KeyVal::I),
            106 => Some(KeyVal::J),
            107 => Some(KeyVal::K),
            108 => Some(KeyVal::L),
            109 => Some(KeyVal::M),
            110 => Some(KeyVal::N),
            111 => Some(KeyVal::O),
            112 => Some(KeyVal::P),
            113 => Some(KeyVal::Q),
            114 => Some(KeyVal::R),
            115 => Some(KeyVal::S),
            116 => Some(KeyVal::T),
            117 => Some(KeyVal::U),
            118 => Some(KeyVal::V),
            119 => Some(KeyVal::W),
            120 => Some(KeyVal::X),
            121 => Some(KeyVal::Y),
            122 => Some(KeyVal::Z),
            44 => Some(KeyVal::Comma),
            46 => Some(KeyVal::Period),
            59 => Some(KeyVal::SemiColon),
            39 => Some(KeyVal::SingleQuote),
            47 => Some(KeyVal::ForwardSlash),
            32 => Some(KeyVal::Space),
            65293 => Some(KeyVal::Enter),
            65505 => Some(KeyVal::Shift),
            45 => Some(KeyVal::Minus),
            61 => Some(KeyVal::Equal),
            65362 => Some(KeyVal::Up),
            65364 => Some(KeyVal::Down),
            65361 => Some(KeyVal::Left),
            65363 => Some(KeyVal::Right),
            65288 => Some(KeyVal::Backspace),
            65307 => Some(KeyVal::Escape),
            48 => Some(KeyVal::_0),
            49 => Some(KeyVal::_1),
            50 => Some(KeyVal::_2),
            51 => Some(KeyVal::_3),
            52 => Some(KeyVal::_4),
            53 => Some(KeyVal::_5),
            54 => Some(KeyVal::_6),
            55 => Some(KeyVal::_7),
            56 => Some(KeyVal::_8),
            57 => Some(KeyVal::_9),
            _ => None,
        }
    }
}

#[allow(dead_code)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Comma,
    Period,
    SemiColon,
    Colon,
    SingleQuote,
    DoubleQuote,
    ForwardSlash,
    QuestionMark,
    Space,
    Enter,
    Shift,
    Minus,
    Equal,
    Up,
    Down,
    Left,
    Right,
    Backspace,
    Escape,
    _0,
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
}