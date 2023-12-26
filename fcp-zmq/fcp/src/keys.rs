#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
enum FcitxKeySym {
    /* Sort by frequency */
    A = 0x0041,         /* U+0041 LATIN CAPITAL LETTER A */
    B = 0x0042,         /* U+0042 LATIN CAPITAL LETTER B */
    C = 0x0043,         /* U+0043 LATIN CAPITAL LETTER C */
    D = 0x0044,         /* U+0044 LATIN CAPITAL LETTER D */
    E = 0x0045,         /* U+0045 LATIN CAPITAL LETTER E */
    F = 0x0046,         /* U+0046 LATIN CAPITAL LETTER F */
    G = 0x0047,         /* U+0047 LATIN CAPITAL LETTER G */
    H = 0x0048,         /* U+0048 LATIN CAPITAL LETTER H */
    I = 0x0049,         /* U+0049 LATIN CAPITAL LETTER I */
    J = 0x004a,         /* U+004A LATIN CAPITAL LETTER J */
    K = 0x004b,         /* U+004B LATIN CAPITAL LETTER K */
    L = 0x004c,         /* U+004C LATIN CAPITAL LETTER L */
    M = 0x004d,         /* U+004D LATIN CAPITAL LETTER M */
    N = 0x004e,         /* U+004E LATIN CAPITAL LETTER N */
    O = 0x004f,         /* U+004F LATIN CAPITAL LETTER O */
    P = 0x0050,         /* U+0050 LATIN CAPITAL LETTER P */
    Q = 0x0051,         /* U+0051 LATIN CAPITAL LETTER Q */
    R = 0x0052,         /* U+0052 LATIN CAPITAL LETTER R */
    S = 0x0053,         /* U+0053 LATIN CAPITAL LETTER S */
    T = 0x0054,         /* U+0054 LATIN CAPITAL LETTER T */
    U = 0x0055,         /* U+0055 LATIN CAPITAL LETTER U */
    V = 0x0056,         /* U+0056 LATIN CAPITAL LETTER V */
    W = 0x0057,         /* U+0057 LATIN CAPITAL LETTER W */
    X = 0x0058,         /* U+0058 LATIN CAPITAL LETTER X */
    Y = 0x0059,         /* U+0059 LATIN CAPITAL LETTER Y */
    Z = 0x005a,         /* U+005A LATIN CAPITAL LETTER Z */
    a = 0x0061,         /* U+0061 LATIN SMALL LETTER A */
    b = 0x0062,         /* U+0062 LATIN SMALL LETTER B */
    c = 0x0063,         /* U+0063 LATIN SMALL LETTER C */
    d = 0x0064,         /* U+0064 LATIN SMALL LETTER D */
    e = 0x0065,         /* U+0065 LATIN SMALL LETTER E */
    f = 0x0066,         /* U+0066 LATIN SMALL LETTER F */
    g = 0x0067,         /* U+0067 LATIN SMALL LETTER G */
    h = 0x0068,         /* U+0068 LATIN SMALL LETTER H */
    i = 0x0069,         /* U+0069 LATIN SMALL LETTER I */
    j = 0x006a,         /* U+006A LATIN SMALL LETTER J */
    k = 0x006b,         /* U+006B LATIN SMALL LETTER K */
    l = 0x006c,         /* U+006C LATIN SMALL LETTER L */
    m = 0x006d,         /* U+006D LATIN SMALL LETTER M */
    n = 0x006e,         /* U+006E LATIN SMALL LETTER N */
    o = 0x006f,         /* U+006F LATIN SMALL LETTER O */
    p = 0x0070,         /* U+0070 LATIN SMALL LETTER P */
    q = 0x0071,         /* U+0071 LATIN SMALL LETTER Q */
    r = 0x0072,         /* U+0072 LATIN SMALL LETTER R */
    s = 0x0073,         /* U+0073 LATIN SMALL LETTER S */
    t = 0x0074,         /* U+0074 LATIN SMALL LETTER T */
    u = 0x0075,         /* U+0075 LATIN SMALL LETTER U */
    v = 0x0076,         /* U+0076 LATIN SMALL LETTER V */
    w = 0x0077,         /* U+0077 LATIN SMALL LETTER W */
    x = 0x0078,         /* U+0078 LATIN SMALL LETTER X */
    y = 0x0079,         /* U+0079 LATIN SMALL LETTER Y */
    z = 0x007a,         /* U+007A LATIN SMALL LETTER Z */
    Num0 = 0x0030,      /* U+0030 DIGIT ZERO */
    Num1 = 0x0031,      /* U+0031 DIGIT ONE */
    Num2 = 0x0032,      /* U+0032 DIGIT TWO */
    Num3 = 0x0033,      /* U+0033 DIGIT THREE */
    Num4 = 0x0034,      /* U+0034 DIGIT FOUR */
    Num5 = 0x0035,      /* U+0035 DIGIT FIVE */
    Num6 = 0x0036,      /* U+0036 DIGIT SIX */
    Num7 = 0x0037,      /* U+0037 DIGIT SEVEN */
    Num8 = 0x0038,      /* U+0038 DIGIT EIGHT */
    Num9 = 0x0039,      /* U+0039 DIGIT NINE */
    Space = 0x0020,     /* U+0020 SPACE */
    BackSpace = 0xff08, /* Back space, back char */
    Return = 0xff0d,    /* Return, enter */
    Escape = 0xff1b,
    Period = 0x002e,               /* U+002E FULL STOP */
    Comma = 0x002c,                /* U+002C COMMA */
    DoubleQuote = 0x0022,          /* U+0022 QUOTATION MARK */
    Question = 0x003f,             /* U+003F QUESTION MARK */
    Exclam = 0x0021,               /* U+0021 EXCLAMATION MARK */
    ParenLeft = 0x0028,            /* U+0028 LEFT PARENTHESIS */
    ParenRight = 0x0029,           /* U+0029 RIGHT PARENTHESIS */
    Equal = 0x003d,                /* U+003D EQUALS SIGN */
    Minus = 0x002d,                /* U+002D HYPHEN-MINUS */
    Slash = 0x002f,                /* U+002F SOLIDUS */
    Colon = 0x003a,                /* U+003A COLON */
    Semicolon = 0x003b,            /* U+003B SEMICOLON */
    Left = 0xff51,                 /* Move left, left arrow */
    Up = 0xff52,                   /* Move up, up arrow */
    Right = 0xff53,                /* Move right, right arrow */
    Down = 0xff54,                 /* Move down, down arrow */
    Less = 0x003c,                 /* U+003C LESS-THAN SIGN */
    Greater = 0x003e,              /* U+003E GREATER-THAN SIGN */
    LeftSingleQuoteMark = 0x0ad0,  /* U+2018 LEFT SINGLE QUOTATION MARK */
    RightSingleQuoteMark = 0x0ad1, /* U+2019 RIGHT SINGLE QUOTATION MARK */
    Ellipsis = 0x0aae,             /* U+2026 HORIZONTAL ELLIPSIS */
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
    F13 = 0xffca,
    F14 = 0xffcb,
    F15 = 0xffcc,
    F16 = 0xffcd,
    F17 = 0xffce,
    F18 = 0xffcf,
    F19 = 0xffd0,
    F20 = 0xffd1,
    F21 = 0xffd2,
    F22 = 0xffd3,
    F23 = 0xffd4,
    F24 = 0xffd5,
    F25 = 0xffd6,
    F26 = 0xffd7,
    F27 = 0xffd8,
    F28 = 0xffd9,
    F29 = 0xffda,
    F30 = 0xffdb,
    F31 = 0xffdc,
    F32 = 0xffdd,
    F33 = 0xffde,
    F34 = 0xffdf,
    F35 = 0xffe0,
    /* Sort alphabetically */
    Acute = 0x00b4,      /* U+00B4 ACUTE ACCENT */
    AltL = 0xffe9,       /* Left alt */
    AltR = 0xffea,       /* Right alt */
    Ampersand = 0x0026,  /* U+0026 AMPERSAND */
    Apostrophe = 0x0027, /* U+0027 APOSTROPHE */
    AsciiTilde = 0x007e, /* U+007E TILDE */
    Asterisk = 0x002a,   /* U+002A ASTERISK */
    At = 0x0040,         /* U+0040 COMMERCIAL AT */
    Bar = 0x007c,        /* U+007C VERTICAL LINE */
    Begin = 0xff58,      /* BOL */
    Blank = 0x09df,
    BraceLeft = 0x007b,    /* U+007B LEFT CURLY BRACKET */
    BraceRight = 0x007d,   /* U+007D RIGHT CURLY BRACKET */
    BracketLeft = 0x005b,  /* U+005B LEFT SQUARE BRACKET */
    BracketRight = 0x005d, /* U+005D RIGHT SQUARE BRACKET */
    Break = 0xff6b,
    Cancel = 0xff69,   /* Cancel, stop, abort, exit */
    CapsLock = 0xffe5, /* Caps lock */
    Caret = 0x0afc,    /* U+2038 CARET */
    Cent = 0x00a2,     /* U+00A2 CENT SIGN */
    Clear = 0xff0b,
    CodeInput = 0xff37,
    ColonSign = 0x10020a1,    /* U+20A1 COLON SIGN */
    ControlL = 0xffe3,        /* Left control */
    ControlR = 0xffe4,        /* Right control */
    Copyright = 0x00a9,       /* U+00A9 COPYRIGHT SIGN */
    CruzeiroSign = 0x10020a2, /* U+20A2 CRUZEIRO SIGN */
    Currency = 0x00a4,        /* U+00A4 CURRENCY SIGN */
    Cursor = 0x0aff,
    DecimalOoint = 0x0abd,       /*(U+002E FULL STOP)*/
    Degree = 0x00b0,             /* U+00B0 DEGREE SIGN */
    Delete = 0xffff,             /* Delete, rubout */
    Division = 0x00f7,           /* U+00F7 DIVISION SIGN */
    Dollar = 0x0024,             /* U+0024 DOLLAR SIGN */
    DongSign = 0x10020ab,        /* U+20AB DONG SIGN */
    DoubleLowQuoteMark = 0x0afe, /* U+201E DOUBLE LOW-9 QUOTATION MARK */
    DownArrow = 0x08fe,          /* U+2193 DOWNWARDS ARROW */
    DownCaret = 0x0ba8,          /*(U+2228 LOGICAL OR)*/
    EcuSign = 0x10020a0,         /* U+20A0 EURO-CURRENCY SIGN */
    EmDash = 0x0aa9,             /* U+2014 EM DASH */
    EnDash = 0x0aaa,             /* U+2013 EN DASH */
    End = 0xff57,                /* EOL */
    EuroSign = 0x20ac,           /* U+20AC EURO SIGN */
    ExclamDown = 0x00a1,         /* U+00A1 INVERTED EXCLAMATION MARK */
    Execute = 0xff62,            /* Execute, run, do */
    FFrancSign = 0x10020a3,      /* U+20A3 FRENCH FRANC SIGN */
    Find = 0xff68,               /* Find, search */
    Grave = 0x0060,              /* U+0060 GRAVE ACCENT */
    GuillemotLeft = 0x00ab,      /* U+00AB LEFT-POINTING DOUBLE ANGLE QUOTATION MARK */
    GuillemotRight = 0x00bb,     /* U+00BB RIGHT-POINTING DOUBLE ANGLE QUOTATION MARK */
    Help = 0xff6a,               /* Help */
    Home = 0xff50,
    HyperL = 0xffed,              /* Left hyper */
    HyperR = 0xffee,              /* Right hyper */
    Hyphen = 0x00ad,              /* U+00AD SOFT HYPHEN */
    Insert = 0xff63,              /* Insert, insert here */
    KanaClosingBracket = 0x04a3,  /* U+300D RIGHT CORNER BRACKET */
    KanaComma = 0x04a4,           /* U+3001 IDEOGRAPHIC COMMA */
    KanaFullstop = 0x04a1,        /* U+3002 IDEOGRAPHIC FULL STOP */
    KanaOpeningBracket = 0x04a2,  /* U+300C LEFT CORNER BRACKET */
    KoreanWon = 0x0eff,           /*(U+20A9 WON SIGN)*/
    LeftAngleBracket = 0x0abc,    /*(U+27E8 MATHEMATICAL LEFT ANGLE BRACKET)*/
    LeftArrow = 0x08fb,           /* U+2190 LEFTWARDS ARROW */
    LeftCaret = 0x0ba3,           /*(U+003C LESS-THAN SIGN)*/
    LeftDoubleQuoteMark = 0x0ad2, /* U+201C LEFT DOUBLE QUOTATION MARK */
    Linefeed = 0xff0a,            /* Linefeed, LF */
    LiraSign = 0x10020a4,         /* U+20A4 LIRA SIGN */
    Menu = 0xff67,
    MetaL = 0xffe7,       /* Left meta */
    MetaR = 0xffe8,       /* Right meta */
    MillSign = 0x10020a5, /* U+20A5 MILL SIGN */
    ModeSwitch = 0xff7e,  /* Character set switch */
    MultiKey = 0xff20,    /* Multi-key character compose */
    MultipleCandidate = 0xff3d,
    Multiply = 0x00d7,         /* U+00D7 MULTIPLICATION SIGN */
    NairaSign = 0x10020a6,     /* U+20A6 NAIRA SIGN */
    NewSheqelSign = 0x10020aa, /* U+20AA NEW SHEQEL SIGN */
    Next = 0xff56,             /* Next */
    NobreakSpace = 0x00a0,     /* U+00A0 NO-BREAK SPACE */
    None = 0x0,
    NotSign = 0x00ac, /* U+00AC NOT SIGN */
    NumLock = 0xff7f,
    NumberSign = 0x0023,     /* U+0023 NUMBER SIGN */
    OrdFeminine = 0x00aa,    /* U+00AA FEMININE ORDINAL INDICATOR */
    Pause = 0xff13,          /* Pause, hold */
    Percent = 0x0025,        /* U+0025 PERCENT SIGN */
    PeriodCentered = 0x00b7, /* U+00B7 MIDDLE DOT */
    PesetaSign = 0x10020a7,  /* U+20A7 PESETA SIGN */
    Plus = 0x002b,           /* U+002B PLUS SIGN */
    PlusMinus = 0x00b1,      /* U+00B1 PLUS-MINUS SIGN */
    PreviousCandidate = 0xff3e,
    Print = 0xff61,
    Prior = 0xff55,                /* Prior, previous */
    QuestionDown = 0x00bf,         /* U+00BF INVERTED QUESTION MARK */
    Redo = 0xff66,                 /* Redo, again */
    RightAngleBracket = 0x0abe,    /*(U+27E9 MATHEMATICAL RIGHT ANGLE BRACKET)*/
    RightArrow = 0x08fd,           /* U+2192 RIGHTWARDS ARROW */
    RightCaret = 0x0ba6,           /*(U+003E GREATER-THAN SIGN)*/
    RightDoubleQuoteMark = 0x0ad3, /* U+201D RIGHT DOUBLE QUOTATION MARK */
    RupeeSign = 0x10020a8,         /* U+20A8 RUPEE SIGN */
    ScrollLock = 0xff14,
    Select = 0xff60,    /* Select, mark */
    ShiftL = 0xffe1,    /* Left shift */
    ShiftLock = 0xffe6, /* Shift lock */
    ShiftR = 0xffe2,    /* Right shift */
    SingleCandidate = 0xff3c,
    SingleLowQuoteMark = 0x0afd, /* U+201A SINGLE LOW-9 QUOTATION MARK */
    Sterling = 0x00a3,           /* U+00A3 POUND SIGN */
    SuperL = 0xffeb,             /* Left super */
    SuperR = 0xffec,             /* Right super */
    SysReq = 0xff15,
    Tab = 0xff09,
    Undo = 0xff65,
    UpArrow = 0x08fc,      /* U+2191 UPWARDS ARROW */
    UpCaret = 0x0ba9,      /*(U+2227 LOGICAL AND)*/
    VoidSymbol = 0xffffff, /* Void symbol */
    WonSign = 0x10020a9,   /* U+20A9 WON SIGN */
    Yen = 0x00a5,          /* U+00A5 YEN SIGN */
}

#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Key {
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90,
    a = 97,
    b = 98,
    c = 99,
    d = 100,
    e = 101,
    f = 102,
    g = 103,
    h = 104,
    i = 105,
    j = 106,
    k = 107,
    l = 108,
    m = 109,
    n = 110,
    o = 111,
    p = 112,
    q = 113,
    r = 114,
    s = 115,
    t = 116,
    u = 117,
    v = 118,
    w = 119,
    x = 120,
    y = 121,
    z = 122,
    Comma = 44,
    Period = 46,
    Colon = 58,
    SemiColon = 59,
    DoubleQuote = 34,
    SingleQuote = 39,
    BracketOpen = 40,
    BracketClose = 41,
    QuestionMark = 63,
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
    Shift = 65505,
    Ctrl = 65507,
    Alt = 65513,
    BackSlash = 92,
    ExclamationMark = 33,
    Ellipsis = 94,
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

impl Key {
    pub fn from_u32(num: u32) -> Option<Key> {
        match num {
            97 => Some(Key::a),
            98 => Some(Key::b),
            99 => Some(Key::c),
            100 => Some(Key::d),
            101 => Some(Key::e),
            102 => Some(Key::f),
            103 => Some(Key::g),
            104 => Some(Key::h),
            105 => Some(Key::i),
            106 => Some(Key::j),
            107 => Some(Key::k),
            108 => Some(Key::l),
            109 => Some(Key::m),
            110 => Some(Key::n),
            111 => Some(Key::o),
            112 => Some(Key::p),
            113 => Some(Key::q),
            114 => Some(Key::r),
            115 => Some(Key::s),
            116 => Some(Key::t),
            117 => Some(Key::u),
            118 => Some(Key::v),
            119 => Some(Key::w),
            120 => Some(Key::x),
            121 => Some(Key::y),
            122 => Some(Key::z),
            44 => Some(Key::Comma),
            46 => Some(Key::Period),
            58 => Some(Key::Colon),
            59 => Some(Key::SemiColon),
            32 => Some(Key::Space),
            34 => Some(Key::DoubleQuote),
            39 => Some(Key::SingleQuote),
            40 => Some(Key::BracketOpen),
            41 => Some(Key::BracketClose),
            63 => Some(Key::QuestionMark),
            65293 => Some(Key::Enter),
            45 => Some(Key::Minus),
            61 => Some(Key::Equal),
            65362 => Some(Key::Up),
            65364 => Some(Key::Down),
            65361 => Some(Key::Left),
            65363 => Some(Key::Right),
            65288 => Some(Key::Backspace),
            65307 => Some(Key::Escape),
            65505 => Some(Key::Shift),
            65507 => Some(Key::Ctrl),
            65513 => Some(Key::Alt),
            92 => Some(Key::BackSlash),
            33 => Some(Key::ExclamationMark),
            94 => Some(Key::Ellipsis),
            48 => Some(Key::_0),
            49 => Some(Key::_1),
            50 => Some(Key::_2),
            51 => Some(Key::_3),
            52 => Some(Key::_4),
            53 => Some(Key::_5),
            54 => Some(Key::_6),
            55 => Some(Key::_7),
            56 => Some(Key::_8),
            57 => Some(Key::_9),
            _ => None,
        }
    }

    pub fn to_char(self) -> Option<char> {
        match self {
            Key::a => Some('a'),
            Key::b => Some('b'),
            Key::c => Some('c'),
            Key::d => Some('d'),
            Key::e => Some('e'),
            Key::f => Some('f'),
            Key::g => Some('g'),
            Key::h => Some('h'),
            Key::i => Some('i'),
            Key::j => Some('j'),
            Key::k => Some('k'),
            Key::l => Some('l'),
            Key::m => Some('m'),
            Key::n => Some('n'),
            Key::o => Some('o'),
            Key::p => Some('p'),
            Key::q => Some('q'),
            Key::r => Some('r'),
            Key::s => Some('s'),
            Key::t => Some('t'),
            Key::u => Some('u'),
            Key::v => Some('v'),
            Key::w => Some('w'),
            Key::x => Some('x'),
            Key::y => Some('y'),
            Key::z => Some('z'),
            Key::A => Some('A'),
            Key::B => Some('B'),
            Key::C => Some('C'),
            Key::D => Some('D'),
            Key::E => Some('E'),
            Key::F => Some('F'),
            Key::G => Some('G'),
            Key::H => Some('H'),
            Key::I => Some('I'),
            Key::J => Some('J'),
            Key::K => Some('K'),
            Key::L => Some('L'),
            Key::M => Some('M'),
            Key::N => Some('N'),
            Key::O => Some('O'),
            Key::P => Some('P'),
            Key::Q => Some('Q'),
            Key::R => Some('R'),
            Key::S => Some('S'),
            Key::T => Some('T'),
            Key::U => Some('U'),
            Key::V => Some('V'),
            Key::W => Some('W'),
            Key::X => Some('X'),
            Key::Y => Some('Y'),
            Key::Z => Some('Z'),
            _ => None,
        }
    }

    pub fn to_usize(self) -> Option<usize> {
        match self {
            Key::_0 => Some(0),
            Key::_1 => Some(1),
            Key::_2 => Some(2),
            Key::_3 => Some(3),
            Key::_4 => Some(4),
            Key::_5 => Some(5),
            Key::_6 => Some(6),
            Key::_7 => Some(7),
            Key::_8 => Some(8),
            Key::_9 => Some(9),
            _ => None,
        }
    }

    pub fn to_full_width_string(self) -> Option<String> {
        match self {
            Key::Comma => Some("，".to_owned()),
            Key::Period => Some("。".to_owned()),
            Key::SemiColon => Some("；".to_owned()),
            Key::Colon => Some("：".to_owned()),
            Key::SingleQuote => Some("‘’".to_owned()),
            Key::DoubleQuote => Some("“”".to_owned()),
            Key::QuestionMark => Some("？".to_owned()),
            Key::BracketOpen => Some("（".to_owned()),
            Key::BracketClose => Some("）".to_owned()),
            Key::BackSlash => Some("、".to_owned()),
            Key::ExclamationMark => Some("！".to_owned()),
            Key::Ellipsis => Some("…".to_owned()),
            _ => None,
        }
    }
}
