use dispatcher::Dispatcher;
use ims::Sub;
use keys::Key;

pub mod candidate;
pub mod candidate_service;
pub mod cloud_pinyin_client;
pub mod dispatcher;
pub mod ims;
pub mod ims_recv;
pub mod ims_send;
pub mod keys;
pub mod number_service;
pub mod preedit_service;
pub mod symbol_service;

#[tokio::main]
async fn main() {
    let sub = Sub::new("tcp://127.0.0.1:8085");
    let dispatcher = Dispatcher::new();
    loop {
        let fe = sub.recv();
        let key = match fe.event {
            ims_send::KeyEvent::NUM_0 => Key::_0,
            ims_send::KeyEvent::NUM_1 => Key::_1,
            ims_send::KeyEvent::NUM_2 => Key::_2,
            ims_send::KeyEvent::NUM_3 => Key::_3,
            ims_send::KeyEvent::NUM_4 => Key::_4,
            ims_send::KeyEvent::NUM_5 => Key::_5,
            ims_send::KeyEvent::NUM_6 => Key::_6,
            ims_send::KeyEvent::NUM_7 => Key::_7,
            ims_send::KeyEvent::NUM_8 => Key::_8,
            ims_send::KeyEvent::NUM_9 => Key::_9,
            ims_send::KeyEvent::A_LWR => Key::a,
            ims_send::KeyEvent::B_LWR => Key::b,
            ims_send::KeyEvent::C_LWR => Key::c,
            ims_send::KeyEvent::D_LWR => Key::d,
            ims_send::KeyEvent::E_LWR => Key::e,
            ims_send::KeyEvent::F_LWR => Key::f,
            ims_send::KeyEvent::G_LWR => Key::g,
            ims_send::KeyEvent::H_LWR => Key::h,
            ims_send::KeyEvent::I_LWR => Key::i,
            ims_send::KeyEvent::J_LWR => Key::j,
            ims_send::KeyEvent::K_LWR => Key::k,
            ims_send::KeyEvent::L_LWR => Key::l,
            ims_send::KeyEvent::M_LWR => Key::m,
            ims_send::KeyEvent::N_LWR => Key::n,
            ims_send::KeyEvent::O_LWR => Key::o,
            ims_send::KeyEvent::P_LWR => Key::p,
            ims_send::KeyEvent::Q_LWR => Key::q,
            ims_send::KeyEvent::R_LWR => Key::r,
            ims_send::KeyEvent::S_LWR => Key::s,
            ims_send::KeyEvent::T_LWR => Key::t,
            ims_send::KeyEvent::U_LWR => Key::u,
            ims_send::KeyEvent::V_LWR => Key::v,
            ims_send::KeyEvent::W_LWR => Key::w,
            ims_send::KeyEvent::X_LWR => Key::x,
            ims_send::KeyEvent::Y_LWR => Key::y,
            ims_send::KeyEvent::Z_LWR => Key::z,
            ims_send::KeyEvent::A_UPR => Key::A,
            ims_send::KeyEvent::B_UPR => Key::B,
            ims_send::KeyEvent::C_UPR => Key::C,
            ims_send::KeyEvent::D_UPR => Key::D,
            ims_send::KeyEvent::E_UPR => Key::E,
            ims_send::KeyEvent::F_UPR => Key::F,
            ims_send::KeyEvent::G_UPR => Key::G,
            ims_send::KeyEvent::H_UPR => Key::H,
            ims_send::KeyEvent::I_UPR => Key::I,
            ims_send::KeyEvent::J_UPR => Key::J,
            ims_send::KeyEvent::K_UPR => Key::K,
            ims_send::KeyEvent::L_UPR => Key::L,
            ims_send::KeyEvent::M_UPR => Key::M,
            ims_send::KeyEvent::N_UPR => Key::N,
            ims_send::KeyEvent::O_UPR => Key::O,
            ims_send::KeyEvent::P_UPR => Key::P,
            ims_send::KeyEvent::Q_UPR => Key::Q,
            ims_send::KeyEvent::R_UPR => Key::R,
            ims_send::KeyEvent::S_UPR => Key::S,
            ims_send::KeyEvent::T_UPR => Key::T,
            ims_send::KeyEvent::U_UPR => Key::U,
            ims_send::KeyEvent::V_UPR => Key::V,
            ims_send::KeyEvent::W_UPR => Key::W,
            ims_send::KeyEvent::X_UPR => Key::X,
            ims_send::KeyEvent::Y_UPR => Key::Y,
            ims_send::KeyEvent::Z_UPR => Key::Z,
            ims_send::KeyEvent::COMMA => Key::Comma,
            ims_send::KeyEvent::PERIOD => Key::Period,
            ims_send::KeyEvent::QEST_MARK => Key::QuestionMark,
            ims_send::KeyEvent::EXCL_MARK => Key::ExclamationMark,
            ims_send::KeyEvent::SEMI_COLON => Key::SemiColon,
            ims_send::KeyEvent::DBL_QUOTE => Key::DoubleQuote,
            ims_send::KeyEvent::SGL_QUOTE => Key::SingleQuote,
            ims_send::KeyEvent::BRKT_OPEN => Key::BracketOpen,
            ims_send::KeyEvent::BRKT_CLOSE => Key::BracketClose,
            ims_send::KeyEvent::SLASH => Key::BackSlash, // We don't have Slash.
            ims_send::KeyEvent::BACKSLASH => Key::BackSlash,
            ims_send::KeyEvent::ELLIPSIS => Key::Ellipsis,
            ims_send::KeyEvent::ENTER => Key::Enter,
            ims_send::KeyEvent::SPACE => Key::Space,
            ims_send::KeyEvent::MINUS => Key::Minus,
            ims_send::KeyEvent::EQUAL => Key::Equal,
            ims_send::KeyEvent::UP => Key::Up,
            ims_send::KeyEvent::DOWN => Key::Down,
            ims_send::KeyEvent::LEFT => Key::Left,
            ims_send::KeyEvent::RIGHT => Key::Right,
            ims_send::KeyEvent::SHIFT => Key::Shift,
            ims_send::KeyEvent::CTRL => Key::Ctrl,
            ims_send::KeyEvent::ALT => Key::Alt,
        };
        _ = dispatcher.on_input(key).await;
    }
}
