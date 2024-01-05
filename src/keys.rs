#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FcitxKeySym {
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
    Backspace = 0xff08, /* Back space, back char */
    Return = 0xff0d,    /* Return, enter */
    Escape = 0xff1b,
    Period = 0x002e,      /* U+002E FULL STOP */
    Comma = 0x002c,       /* U+002C COMMA */
    DoubleQuote = 0x0022, /* U+0022 QUOTATION MARK */
    Question = 0x003f,    /* U+003F QUESTION MARK */
    Exclam = 0x0021,      /* U+0021 EXCLAMATION MARK */
    ParenLeft = 0x0028,   /* U+0028 LEFT PARENTHESIS */
    ParenRight = 0x0029,  /* U+0029 RIGHT PARENTHESIS */
    Equal = 0x003d,       /* U+003D EQUALS SIGN */
    Minus = 0x002d,       /* U+002D HYPHEN-MINUS */
    Slash = 0x002f,       /* U+002F SOLIDUS */
    Colon = 0x003a,       /* U+003A COLON */
    Semicolon = 0x003b,   /* U+003B SEMICOLON */
    Backslash = 0x005c,
    Left = 0xff51,                 /* Move left, left arrow */
    Up = 0xff52,                   /* Move up, up arrow */
    Right = 0xff53,                /* Move right, right arrow */
    Down = 0xff54,                 /* Move down, down arrow */
    Less = 0x003c,                 /* U+003C LESS-THAN SIGN */
    Greater = 0x003e,              /* U+003E GREATER-THAN SIGN */
    LeftSingleQuoteMark = 0x0ad0,  /* U+2018 LEFT SINGLE QUOTATION MARK */
    RightSingleQuoteMark = 0x0ad1, /* U+2019 RIGHT SINGLE QUOTATION MARK */
    Ellipsis = 0x0aae,             /* U+2026 HORIZONTAL ELLIPSIS */
    Asciicircum = 0x005e,
    Underscore = 0x005f,
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

impl FcitxKeySym {
    pub fn from_u32(n: u32) -> Option<FcitxKeySym> {
        match n {
            0x0041 => Some(FcitxKeySym::A),
            0x0042 => Some(FcitxKeySym::B),
            0x0043 => Some(FcitxKeySym::C),
            0x0044 => Some(FcitxKeySym::D),
            0x0045 => Some(FcitxKeySym::E),
            0x0046 => Some(FcitxKeySym::F),
            0x0047 => Some(FcitxKeySym::G),
            0x0048 => Some(FcitxKeySym::H),
            0x0049 => Some(FcitxKeySym::I),
            0x004a => Some(FcitxKeySym::J),
            0x004b => Some(FcitxKeySym::K),
            0x004c => Some(FcitxKeySym::L),
            0x004d => Some(FcitxKeySym::M),
            0x004e => Some(FcitxKeySym::N),
            0x004f => Some(FcitxKeySym::O),
            0x0050 => Some(FcitxKeySym::P),
            0x0051 => Some(FcitxKeySym::Q),
            0x0052 => Some(FcitxKeySym::R),
            0x0053 => Some(FcitxKeySym::S),
            0x0054 => Some(FcitxKeySym::T),
            0x0055 => Some(FcitxKeySym::U),
            0x0056 => Some(FcitxKeySym::V),
            0x0057 => Some(FcitxKeySym::W),
            0x0058 => Some(FcitxKeySym::X),
            0x0059 => Some(FcitxKeySym::Y),
            0x005a => Some(FcitxKeySym::Z),
            0x0061 => Some(FcitxKeySym::a),
            0x0062 => Some(FcitxKeySym::b),
            0x0063 => Some(FcitxKeySym::c),
            0x0064 => Some(FcitxKeySym::d),
            0x0065 => Some(FcitxKeySym::e),
            0x0066 => Some(FcitxKeySym::f),
            0x0067 => Some(FcitxKeySym::g),
            0x0068 => Some(FcitxKeySym::h),
            0x0069 => Some(FcitxKeySym::i),
            0x006a => Some(FcitxKeySym::j),
            0x006b => Some(FcitxKeySym::k),
            0x006c => Some(FcitxKeySym::l),
            0x006d => Some(FcitxKeySym::m),
            0x006e => Some(FcitxKeySym::n),
            0x006f => Some(FcitxKeySym::o),
            0x0070 => Some(FcitxKeySym::p),
            0x0071 => Some(FcitxKeySym::q),
            0x0072 => Some(FcitxKeySym::r),
            0x0073 => Some(FcitxKeySym::s),
            0x0074 => Some(FcitxKeySym::t),
            0x0075 => Some(FcitxKeySym::u),
            0x0076 => Some(FcitxKeySym::v),
            0x0077 => Some(FcitxKeySym::w),
            0x0078 => Some(FcitxKeySym::x),
            0x0079 => Some(FcitxKeySym::y),
            0x007a => Some(FcitxKeySym::z),
            0x0030 => Some(FcitxKeySym::Num0),
            0x0031 => Some(FcitxKeySym::Num1),
            0x0032 => Some(FcitxKeySym::Num2),
            0x0033 => Some(FcitxKeySym::Num3),
            0x0034 => Some(FcitxKeySym::Num4),
            0x0035 => Some(FcitxKeySym::Num5),
            0x0036 => Some(FcitxKeySym::Num6),
            0x0037 => Some(FcitxKeySym::Num7),
            0x0038 => Some(FcitxKeySym::Num8),
            0x0039 => Some(FcitxKeySym::Num9),
            0x0020 => Some(FcitxKeySym::Space),
            0xff08 => Some(FcitxKeySym::Backspace),
            0xff0d => Some(FcitxKeySym::Return),
            0xff1b => Some(FcitxKeySym::Escape),
            0x002e => Some(FcitxKeySym::Period),
            0x002c => Some(FcitxKeySym::Comma),
            0x0022 => Some(FcitxKeySym::DoubleQuote),
            0x003f => Some(FcitxKeySym::Question),
            0x0021 => Some(FcitxKeySym::Exclam),
            0x0028 => Some(FcitxKeySym::ParenLeft),
            0x0029 => Some(FcitxKeySym::ParenRight),
            0x003d => Some(FcitxKeySym::Equal),
            0x002d => Some(FcitxKeySym::Minus),
            0x002f => Some(FcitxKeySym::Slash),
            0x003a => Some(FcitxKeySym::Colon),
            0x003b => Some(FcitxKeySym::Semicolon),
            0x005c => Some(FcitxKeySym::Backslash),
            0xff51 => Some(FcitxKeySym::Left),
            0xff52 => Some(FcitxKeySym::Up),
            0xff53 => Some(FcitxKeySym::Right),
            0xff54 => Some(FcitxKeySym::Down),
            0x003c => Some(FcitxKeySym::Less),
            0x003e => Some(FcitxKeySym::Greater),
            0x0ad0 => Some(FcitxKeySym::LeftSingleQuoteMark),
            0x0ad1 => Some(FcitxKeySym::RightSingleQuoteMark),
            0x005e => Some(FcitxKeySym::Asciicircum),
            0x0aae => Some(FcitxKeySym::Ellipsis),
            0x005f => Some(FcitxKeySym::Underscore),
            0xffbe => Some(FcitxKeySym::F1),
            0xffbf => Some(FcitxKeySym::F2),
            0xffc0 => Some(FcitxKeySym::F3),
            0xffc1 => Some(FcitxKeySym::F4),
            0xffc2 => Some(FcitxKeySym::F5),
            0xffc3 => Some(FcitxKeySym::F6),
            0xffc4 => Some(FcitxKeySym::F7),
            0xffc5 => Some(FcitxKeySym::F8),
            0xffc6 => Some(FcitxKeySym::F9),
            0xffc7 => Some(FcitxKeySym::F10),
            0xffc8 => Some(FcitxKeySym::F11),
            0xffc9 => Some(FcitxKeySym::F12),
            0xffca => Some(FcitxKeySym::F13),
            0xffcb => Some(FcitxKeySym::F14),
            0xffcc => Some(FcitxKeySym::F15),
            0xffcd => Some(FcitxKeySym::F16),
            0xffce => Some(FcitxKeySym::F17),
            0xffcf => Some(FcitxKeySym::F18),
            0xffd0 => Some(FcitxKeySym::F19),
            0xffd1 => Some(FcitxKeySym::F20),
            0xffd2 => Some(FcitxKeySym::F21),
            0xffd3 => Some(FcitxKeySym::F22),
            0xffd4 => Some(FcitxKeySym::F23),
            0xffd5 => Some(FcitxKeySym::F24),
            0xffd6 => Some(FcitxKeySym::F25),
            0xffd7 => Some(FcitxKeySym::F26),
            0xffd8 => Some(FcitxKeySym::F27),
            0xffd9 => Some(FcitxKeySym::F28),
            0xffda => Some(FcitxKeySym::F29),
            0xffdb => Some(FcitxKeySym::F30),
            0xffdc => Some(FcitxKeySym::F31),
            0xffdd => Some(FcitxKeySym::F32),
            0xffde => Some(FcitxKeySym::F33),
            0xffdf => Some(FcitxKeySym::F34),
            0xffe0 => Some(FcitxKeySym::F35),
            0x00b4 => Some(FcitxKeySym::Acute),
            0xffe9 => Some(FcitxKeySym::AltL),
            0xffea => Some(FcitxKeySym::AltR),
            0x0026 => Some(FcitxKeySym::Ampersand),
            0x0027 => Some(FcitxKeySym::Apostrophe),
            0x007e => Some(FcitxKeySym::AsciiTilde),
            0x002a => Some(FcitxKeySym::Asterisk),
            0x0040 => Some(FcitxKeySym::At),
            0x007c => Some(FcitxKeySym::Bar),
            0xff58 => Some(FcitxKeySym::Begin),
            0x09df => Some(FcitxKeySym::Blank),
            0x007b => Some(FcitxKeySym::BraceLeft),
            0x007d => Some(FcitxKeySym::BraceRight),
            0x005b => Some(FcitxKeySym::BracketLeft),
            0x005d => Some(FcitxKeySym::BracketRight),
            0xff6b => Some(FcitxKeySym::Break),
            0xff69 => Some(FcitxKeySym::Cancel),
            0xffe5 => Some(FcitxKeySym::CapsLock),
            0x0afc => Some(FcitxKeySym::Caret),
            0x00a2 => Some(FcitxKeySym::Cent),
            0xff0b => Some(FcitxKeySym::Clear),
            0xff37 => Some(FcitxKeySym::CodeInput),
            0x10020a1 => Some(FcitxKeySym::ColonSign),
            0xffe3 => Some(FcitxKeySym::ControlL),
            0xffe4 => Some(FcitxKeySym::ControlR),
            0x00a9 => Some(FcitxKeySym::Copyright),
            0x10020a2 => Some(FcitxKeySym::CruzeiroSign),
            0x00a4 => Some(FcitxKeySym::Currency),
            0x0aff => Some(FcitxKeySym::Cursor),
            0x0abd => Some(FcitxKeySym::DecimalOoint),
            0x00b0 => Some(FcitxKeySym::Degree),
            0xffff => Some(FcitxKeySym::Delete),
            0x00f7 => Some(FcitxKeySym::Division),
            0x0024 => Some(FcitxKeySym::Dollar),
            0x10020ab => Some(FcitxKeySym::DongSign),
            0x0afe => Some(FcitxKeySym::DoubleLowQuoteMark),
            0x08fe => Some(FcitxKeySym::DownArrow),
            0x0ba8 => Some(FcitxKeySym::DownCaret),
            0x10020a0 => Some(FcitxKeySym::EcuSign),
            0x0aa9 => Some(FcitxKeySym::EmDash),
            0x0aaa => Some(FcitxKeySym::EnDash),
            0xff57 => Some(FcitxKeySym::End),
            0x20ac => Some(FcitxKeySym::EuroSign),
            0x00a1 => Some(FcitxKeySym::ExclamDown),
            0xff62 => Some(FcitxKeySym::Execute),
            0x10020a3 => Some(FcitxKeySym::FFrancSign),
            0xff68 => Some(FcitxKeySym::Find),
            0x0060 => Some(FcitxKeySym::Grave),
            0x00ab => Some(FcitxKeySym::GuillemotLeft),
            0x00bb => Some(FcitxKeySym::GuillemotRight),
            0xff6a => Some(FcitxKeySym::Help),
            0xff50 => Some(FcitxKeySym::Home),
            0xffed => Some(FcitxKeySym::HyperL),
            0xffee => Some(FcitxKeySym::HyperR),
            0x00ad => Some(FcitxKeySym::Hyphen),
            0xff63 => Some(FcitxKeySym::Insert),
            0x04a3 => Some(FcitxKeySym::KanaClosingBracket),
            0x04a4 => Some(FcitxKeySym::KanaComma),
            0x04a1 => Some(FcitxKeySym::KanaFullstop),
            0x04a2 => Some(FcitxKeySym::KanaOpeningBracket),
            0x0eff => Some(FcitxKeySym::KoreanWon),
            0x0abc => Some(FcitxKeySym::LeftAngleBracket),
            0x08fb => Some(FcitxKeySym::LeftArrow),
            0x0ba3 => Some(FcitxKeySym::LeftCaret),
            0x0ad2 => Some(FcitxKeySym::LeftDoubleQuoteMark),
            0xff0a => Some(FcitxKeySym::Linefeed),
            0x10020a4 => Some(FcitxKeySym::LiraSign),
            0xff67 => Some(FcitxKeySym::Menu),
            0xffe7 => Some(FcitxKeySym::MetaL),
            0xffe8 => Some(FcitxKeySym::MetaR),
            0x10020a5 => Some(FcitxKeySym::MillSign),
            0xff7e => Some(FcitxKeySym::ModeSwitch),
            0xff20 => Some(FcitxKeySym::MultiKey),
            0xff3d => Some(FcitxKeySym::MultipleCandidate),
            0x00d7 => Some(FcitxKeySym::Multiply),
            0x10020a6 => Some(FcitxKeySym::NairaSign),
            0x10020aa => Some(FcitxKeySym::NewSheqelSign),
            0xff56 => Some(FcitxKeySym::Next),
            0x00a0 => Some(FcitxKeySym::NobreakSpace),
            0x0 => Some(FcitxKeySym::None),
            0x00ac => Some(FcitxKeySym::NotSign),
            0xff7f => Some(FcitxKeySym::NumLock),
            0x0023 => Some(FcitxKeySym::NumberSign),
            0x00aa => Some(FcitxKeySym::OrdFeminine),
            0xff13 => Some(FcitxKeySym::Pause),
            0x0025 => Some(FcitxKeySym::Percent),
            0x00b7 => Some(FcitxKeySym::PeriodCentered),
            0x10020a7 => Some(FcitxKeySym::PesetaSign),
            0x002b => Some(FcitxKeySym::Plus),
            0x00b1 => Some(FcitxKeySym::PlusMinus),
            0xff3e => Some(FcitxKeySym::PreviousCandidate),
            0xff61 => Some(FcitxKeySym::Print),
            0xff55 => Some(FcitxKeySym::Prior),
            0x00bf => Some(FcitxKeySym::QuestionDown),
            0xff66 => Some(FcitxKeySym::Redo),
            0x0abe => Some(FcitxKeySym::RightAngleBracket),
            0x08fd => Some(FcitxKeySym::RightArrow),
            0x0ba6 => Some(FcitxKeySym::RightCaret),
            0x0ad3 => Some(FcitxKeySym::RightDoubleQuoteMark),
            0x10020a8 => Some(FcitxKeySym::RupeeSign),
            0xff14 => Some(FcitxKeySym::ScrollLock),
            0xff60 => Some(FcitxKeySym::Select),
            0xffe1 => Some(FcitxKeySym::ShiftL),
            0xffe6 => Some(FcitxKeySym::ShiftLock),
            0xffe2 => Some(FcitxKeySym::ShiftR),
            0xff3c => Some(FcitxKeySym::SingleCandidate),
            0x0afd => Some(FcitxKeySym::SingleLowQuoteMark),
            0x00a3 => Some(FcitxKeySym::Sterling),
            0xffeb => Some(FcitxKeySym::SuperL),
            0xffec => Some(FcitxKeySym::SuperR),
            0xff15 => Some(FcitxKeySym::SysReq),
            0xff09 => Some(FcitxKeySym::Tab),
            0xff65 => Some(FcitxKeySym::Undo),
            0x08fc => Some(FcitxKeySym::UpArrow),
            0x0ba9 => Some(FcitxKeySym::UpCaret),
            0xffffff => Some(FcitxKeySym::VoidSymbol),
            0x10020a9 => Some(FcitxKeySym::WonSign),
            0x00a5 => Some(FcitxKeySym::Yen),
            _ => None,
        }
    }

    pub fn to_char(self) -> Option<char> {
        match self {
            FcitxKeySym::a => Some('a'),
            FcitxKeySym::b => Some('b'),
            FcitxKeySym::c => Some('c'),
            FcitxKeySym::d => Some('d'),
            FcitxKeySym::e => Some('e'),
            FcitxKeySym::f => Some('f'),
            FcitxKeySym::g => Some('g'),
            FcitxKeySym::h => Some('h'),
            FcitxKeySym::i => Some('i'),
            FcitxKeySym::j => Some('j'),
            FcitxKeySym::k => Some('k'),
            FcitxKeySym::l => Some('l'),
            FcitxKeySym::m => Some('m'),
            FcitxKeySym::n => Some('n'),
            FcitxKeySym::o => Some('o'),
            FcitxKeySym::p => Some('p'),
            FcitxKeySym::q => Some('q'),
            FcitxKeySym::r => Some('r'),
            FcitxKeySym::s => Some('s'),
            FcitxKeySym::t => Some('t'),
            FcitxKeySym::u => Some('u'),
            FcitxKeySym::v => Some('v'),
            FcitxKeySym::w => Some('w'),
            FcitxKeySym::x => Some('x'),
            FcitxKeySym::y => Some('y'),
            FcitxKeySym::z => Some('z'),
            FcitxKeySym::A => Some('A'),
            FcitxKeySym::B => Some('B'),
            FcitxKeySym::C => Some('C'),
            FcitxKeySym::D => Some('D'),
            FcitxKeySym::E => Some('E'),
            FcitxKeySym::F => Some('F'),
            FcitxKeySym::G => Some('G'),
            FcitxKeySym::H => Some('H'),
            FcitxKeySym::I => Some('I'),
            FcitxKeySym::J => Some('J'),
            FcitxKeySym::K => Some('K'),
            FcitxKeySym::L => Some('L'),
            FcitxKeySym::M => Some('M'),
            FcitxKeySym::N => Some('N'),
            FcitxKeySym::O => Some('O'),
            FcitxKeySym::P => Some('P'),
            FcitxKeySym::Q => Some('Q'),
            FcitxKeySym::R => Some('R'),
            FcitxKeySym::S => Some('S'),
            FcitxKeySym::T => Some('T'),
            FcitxKeySym::U => Some('U'),
            FcitxKeySym::V => Some('V'),
            FcitxKeySym::W => Some('W'),
            FcitxKeySym::X => Some('X'),
            FcitxKeySym::Y => Some('Y'),
            FcitxKeySym::Z => Some('Z'),
            _ => None,
        }
    }

    pub fn to_usize(self) -> Option<usize> {
        match self {
            FcitxKeySym::Num0 => Some(0),
            FcitxKeySym::Num1 => Some(1),
            FcitxKeySym::Num2 => Some(2),
            FcitxKeySym::Num3 => Some(3),
            FcitxKeySym::Num4 => Some(4),
            FcitxKeySym::Num5 => Some(5),
            FcitxKeySym::Num6 => Some(6),
            FcitxKeySym::Num7 => Some(7),
            FcitxKeySym::Num8 => Some(8),
            FcitxKeySym::Num9 => Some(9),
            _ => None,
        }
    }

    pub fn to_full_width_string(self) -> Option<String> {
        match self {
            FcitxKeySym::Comma => Some("，".to_owned()),
            FcitxKeySym::Period => Some("。".to_owned()),
            FcitxKeySym::Semicolon => Some("；".to_owned()),
            FcitxKeySym::Colon => Some("：".to_owned()),
            FcitxKeySym::LeftSingleQuoteMark => Some("‘’".to_owned()),
            FcitxKeySym::RightSingleQuoteMark => Some(("’").to_owned()),
            FcitxKeySym::DoubleQuote => Some("“”".to_owned()),
            FcitxKeySym::Question => Some("？".to_owned()),
            FcitxKeySym::ParenLeft => Some("（".to_owned()),
            FcitxKeySym::ParenRight => Some("）".to_owned()),
            FcitxKeySym::BracketLeft => Some("【".to_owned()),
            FcitxKeySym::BracketRight => Some("】".to_owned()),
            FcitxKeySym::BraceLeft => Some("「".to_owned()),
            FcitxKeySym::BraceRight => Some("」".to_owned()),
            FcitxKeySym::Backslash => Some("、".to_owned()),
            FcitxKeySym::Exclam => Some("！".to_owned()),
            FcitxKeySym::Asciicircum => Some("…".to_owned()),
            FcitxKeySym::Less => Some("《".to_owned()),
            FcitxKeySym::Greater => Some("》".to_owned()),
            _ => None,
        }
    }
}
