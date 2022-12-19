use std::sync::Mutex;

use crate::ffi::{FnCanPageUp, FnCommit, FnSetCandidates, FnSetPage, FnSetPreedit, FnVoid};

pub struct Fcitx5 {
    fn_ptrs: Mutex<Fcitx5FnPtrs>,
}

impl Fcitx5 {
    pub fn new(from_cpp: Fcitx5FnPtrs) -> Self {
        Fcitx5 {
            fn_ptrs: Mutex::new(from_cpp),
        }
    }

    pub fn ui_set_loading(&self) {
        let fn_ptr_mtx = &self.fn_ptrs.lock().expect("Failed to lock fn_ptrs.");
        unsafe {
            (fn_ptr_mtx.ui.set_loading)();
        }
    }

    pub fn ui_set_candidates(&self) {
        let fn_ptr_mtx = &self.fn_ptrs.lock().expect("Failed to lock fn_ptrs.");
        unsafe {
            (fn_ptr_mtx.ui.clear_candidates)();
        }
    }

    pub fn ui_clear_candidates() {}

    pub fn ui_set_preedit() {}

    pub fn table_can_page_up(&self) -> bool {
        let fn_ptr_mtx = &self.fn_ptrs.lock().expect("Failed to lock fn_ptrs.");
        unsafe {
            (fn_ptr_mtx.table.can_page_up)()
        }
    }

    pub fn table_page_up(&self) {
        let fn_ptr_mtx = &self.fn_ptrs.lock().expect("Failed to lock fn_ptrs.");
        unsafe {
            (fn_ptr_mtx.table.page_up)();
        }
    }

    pub fn table_page_down(&self) {
        let fn_ptr_mtx = &self.fn_ptrs.lock().expect("Failed to lock fn_ptrs.");
        unsafe {
            (fn_ptr_mtx.table.page_down)();
        }
    }

    pub fn table_prev(&self) {
        let fn_ptr_mtx = &self.fn_ptrs.lock().expect("Failed to lock fn_ptrs.");
        unsafe {
            (fn_ptr_mtx.table.prev)();
        }
    }

    pub fn table_next(&self) {
        let fn_ptr_mtx = &self.fn_ptrs.lock().expect("Failed to lock fn_ptrs.");
        unsafe {
            (fn_ptr_mtx.table.next)();
        }
    }

    pub fn table_set_page() {}

    pub fn engine_commit() {}

    pub fn engine_commit_preedit() {}

    pub fn engine_commit_candidate_by_fixed_key() {}
}

#[derive(Clone)]
pub struct Fcitx5FnPtrs {
    pub ui: UI,
    pub table: Table,
    pub engine: Engine,
}

#[derive(Clone)]
pub struct UI {
    pub set_loading: FnVoid,
    pub set_candidates: FnSetCandidates,
    pub clear_candidates: FnVoid,
    pub set_preedit: FnSetPreedit,
}

#[derive(Clone)]
pub struct Table {
    pub can_page_up: FnCanPageUp,
    pub page_up: FnVoid,
    pub page_down: FnVoid,
    pub prev: FnVoid,
    pub next: FnVoid,
    pub set_page: FnSetPage,
}

#[derive(Clone)]
pub struct Engine {
    pub commit: FnCommit,
    pub commit_preedit: FnSetPreedit,
    pub commit_candidate_by_fixed_key: FnVoid,
}

#[allow(non_camel_case_types)]
#[allow(dead_code)]
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
    Num0 = 0x0030,         /* U+0030 DIGIT ZERO */
    Num1 = 0x0031,         /* U+0031 DIGIT ONE */
    Num2 = 0x0032,         /* U+0032 DIGIT TWO */
    Num3 = 0x0033,         /* U+0033 DIGIT THREE */
    Num4 = 0x0034,         /* U+0034 DIGIT FOUR */
    Num5 = 0x0035,         /* U+0035 DIGIT FIVE */
    Num6 = 0x0036,         /* U+0036 DIGIT SIX */
    Num7 = 0x0037,         /* U+0037 DIGIT SEVEN */
    Num8 = 0x0038,         /* U+0038 DIGIT EIGHT */
    Num9 = 0x0039,         /* U+0039 DIGIT NINE */
    Colon = 0x003a,        /* U+003A COLON */
    Semicolon = 0x003b,    /* U+003B SEMICOLON */
    Less = 0x003c,         /* U+003C LESS-THAN SIGN */
    Equal = 0x003d,        /* U+003D EQUALS SIGN */
    Hreater = 0x003e,      /* U+003E GREATER-THAN SIGN */
    Question = 0x003f,     /* U+003F QUESTION MARK */
    At = 0x0040,           /* U+0040 COMMERCIAL AT */
    A = 0x0041,            /* U+0041 LATIN CAPITAL LETTER A */
    B = 0x0042,            /* U+0042 LATIN CAPITAL LETTER B */
    C = 0x0043,            /* U+0043 LATIN CAPITAL LETTER C */
    D = 0x0044,            /* U+0044 LATIN CAPITAL LETTER D */
    E = 0x0045,            /* U+0045 LATIN CAPITAL LETTER E */
    F = 0x0046,            /* U+0046 LATIN CAPITAL LETTER F */
    G = 0x0047,            /* U+0047 LATIN CAPITAL LETTER G */
    H = 0x0048,            /* U+0048 LATIN CAPITAL LETTER H */
    I = 0x0049,            /* U+0049 LATIN CAPITAL LETTER I */
    J = 0x004a,            /* U+004A LATIN CAPITAL LETTER J */
    K = 0x004b,            /* U+004B LATIN CAPITAL LETTER K */
    L = 0x004c,            /* U+004C LATIN CAPITAL LETTER L */
    M = 0x004d,            /* U+004D LATIN CAPITAL LETTER M */
    N = 0x004e,            /* U+004E LATIN CAPITAL LETTER N */
    O = 0x004f,            /* U+004F LATIN CAPITAL LETTER O */
    P = 0x0050,            /* U+0050 LATIN CAPITAL LETTER P */
    Q = 0x0051,            /* U+0051 LATIN CAPITAL LETTER Q */
    R = 0x0052,            /* U+0052 LATIN CAPITAL LETTER R */
    S = 0x0053,            /* U+0053 LATIN CAPITAL LETTER S */
    T = 0x0054,            /* U+0054 LATIN CAPITAL LETTER T */
    U = 0x0055,            /* U+0055 LATIN CAPITAL LETTER U */
    V = 0x0056,            /* U+0056 LATIN CAPITAL LETTER V */
    W = 0x0057,            /* U+0057 LATIN CAPITAL LETTER W */
    X = 0x0058,            /* U+0058 LATIN CAPITAL LETTER X */
    Y = 0x0059,            /* U+0059 LATIN CAPITAL LETTER Y */
    Z = 0x005a,            /* U+005A LATIN CAPITAL LETTER Z */
    BracketLeft = 0x005b,  /* U+005B LEFT SQUARE BRACKET */
    Backslash = 0x005c,    /* U+005C REVERSE SOLIDUS */
    BracketRight = 0x005d, /* U+005D RIGHT SQUARE BRACKET */
    AsciiCircum = 0x005e,  /* U+005E CIRCUMFLEX ACCENT */
    Underscore = 0x005f,   /* U+005F LOW LINE */
    Grave = 0x0060,        /* U+0060 GRAVE ACCENT */
    a = 0x0061,            /* U+0061 LATIN SMALL LETTER A */
    b = 0x0062,            /* U+0062 LATIN SMALL LETTER B */
    c = 0x0063,            /* U+0063 LATIN SMALL LETTER C */
    d = 0x0064,            /* U+0064 LATIN SMALL LETTER D */
    e = 0x0065,            /* U+0065 LATIN SMALL LETTER E */
    f = 0x0066,            /* U+0066 LATIN SMALL LETTER F */
    g = 0x0067,            /* U+0067 LATIN SMALL LETTER G */
    h = 0x0068,            /* U+0068 LATIN SMALL LETTER H */
    i = 0x0069,            /* U+0069 LATIN SMALL LETTER I */
    j = 0x006a,            /* U+006A LATIN SMALL LETTER J */
    k = 0x006b,            /* U+006B LATIN SMALL LETTER K */
    l = 0x006c,            /* U+006C LATIN SMALL LETTER L */
    m = 0x006d,            /* U+006D LATIN SMALL LETTER M */
    n = 0x006e,            /* U+006E LATIN SMALL LETTER N */
    o = 0x006f,            /* U+006F LATIN SMALL LETTER O */
    p = 0x0070,            /* U+0070 LATIN SMALL LETTER P */
    q = 0x0071,            /* U+0071 LATIN SMALL LETTER Q */
    r = 0x0072,            /* U+0072 LATIN SMALL LETTER R */
    s = 0x0073,            /* U+0073 LATIN SMALL LETTER S */
    t = 0x0074,            /* U+0074 LATIN SMALL LETTER T */
    u = 0x0075,            /* U+0075 LATIN SMALL LETTER U */
    v = 0x0076,            /* U+0076 LATIN SMALL LETTER V */
    w = 0x0077,            /* U+0077 LATIN SMALL LETTER W */
    x = 0x0078,            /* U+0078 LATIN SMALL LETTER X */
    y = 0x0079,            /* U+0079 LATIN SMALL LETTER Y */
    z = 0x007a,            /* U+007A LATIN SMALL LETTER Z */
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
