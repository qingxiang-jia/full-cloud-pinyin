use dispatcher::Dispatcher;
use ims::Sub;
use keys::Key;

pub mod candidate;
pub mod candidate_service;
pub mod cloud_pinyin_client;
pub mod dispatcher;
pub mod ims;
pub mod keys;
pub mod msgs;
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
            msgs::KeyEvent::NUM_0 => Key::_0,
            msgs::KeyEvent::NUM_1 => Key::_1,
            msgs::KeyEvent::NUM_2 => Key::_2,
            msgs::KeyEvent::NUM_3 => Key::_3,
            msgs::KeyEvent::NUM_4 => Key::_4,
            msgs::KeyEvent::NUM_5 => Key::_5,
            msgs::KeyEvent::NUM_6 => Key::_6,
            msgs::KeyEvent::NUM_7 => Key::_7,
            msgs::KeyEvent::NUM_8 => Key::_8,
            msgs::KeyEvent::NUM_9 => Key::_9,
            msgs::KeyEvent::A_LWR => Key::a,
            msgs::KeyEvent::B_LWR => Key::b,
            msgs::KeyEvent::C_LWR => Key::c,
            msgs::KeyEvent::D_LWR => Key::d,
            msgs::KeyEvent::E_LWR => Key::e,
            msgs::KeyEvent::F_LWR => Key::f,
            msgs::KeyEvent::G_LWR => Key::g,
            msgs::KeyEvent::H_LWR => Key::h,
            msgs::KeyEvent::I_LWR => Key::i,
            msgs::KeyEvent::J_LWR => Key::j,
            msgs::KeyEvent::K_LWR => Key::k,
            msgs::KeyEvent::L_LWR => Key::l,
            msgs::KeyEvent::M_LWR => Key::m,
            msgs::KeyEvent::N_LWR => Key::n,
            msgs::KeyEvent::O_LWR => Key::o,
            msgs::KeyEvent::P_LWR => Key::p,
            msgs::KeyEvent::Q_LWR => Key::q,
            msgs::KeyEvent::R_LWR => Key::r,
            msgs::KeyEvent::S_LWR => Key::s,
            msgs::KeyEvent::T_LWR => Key::t,
            msgs::KeyEvent::U_LWR => Key::u,
            msgs::KeyEvent::V_LWR => Key::v,
            msgs::KeyEvent::W_LWR => Key::w,
            msgs::KeyEvent::X_LWR => Key::x,
            msgs::KeyEvent::Y_LWR => Key::y,
            msgs::KeyEvent::Z_LWR => Key::z,
            msgs::KeyEvent::A_UPR => Key::A,
            msgs::KeyEvent::B_UPR => Key::B,
            msgs::KeyEvent::C_UPR => Key::C,
            msgs::KeyEvent::D_UPR => Key::D,
            msgs::KeyEvent::E_UPR => Key::E,
            msgs::KeyEvent::F_UPR => Key::F,
            msgs::KeyEvent::G_UPR => Key::G,
            msgs::KeyEvent::H_UPR => Key::H,
            msgs::KeyEvent::I_UPR => Key::I,
            msgs::KeyEvent::J_UPR => Key::J,
            msgs::KeyEvent::K_UPR => Key::K,
            msgs::KeyEvent::L_UPR => Key::L,
            msgs::KeyEvent::M_UPR => Key::M,
            msgs::KeyEvent::N_UPR => Key::N,
            msgs::KeyEvent::O_UPR => Key::O,
            msgs::KeyEvent::P_UPR => Key::P,
            msgs::KeyEvent::Q_UPR => Key::Q,
            msgs::KeyEvent::R_UPR => Key::R,
            msgs::KeyEvent::S_UPR => Key::S,
            msgs::KeyEvent::T_UPR => Key::T,
            msgs::KeyEvent::U_UPR => Key::U,
            msgs::KeyEvent::V_UPR => Key::V,
            msgs::KeyEvent::W_UPR => Key::W,
            msgs::KeyEvent::X_UPR => Key::X,
            msgs::KeyEvent::Y_UPR => Key::Y,
            msgs::KeyEvent::Z_UPR => Key::Z,
            msgs::KeyEvent::COMMA => Key::Comma,
            msgs::KeyEvent::PERIOD => Key::Period,
            msgs::KeyEvent::QEST_MARK => Key::QuestionMark,
            msgs::KeyEvent::EXCL_MARK => Key::ExclamationMark,
            msgs::KeyEvent::SEMI_COLON => Key::SemiColon,
            msgs::KeyEvent::DBL_QUOTE => Key::DoubleQuote,
            msgs::KeyEvent::SGL_QUOTE => Key::SingleQuote,
            msgs::KeyEvent::BRKT_OPEN => Key::BracketOpen,
            msgs::KeyEvent::BRKT_CLOSE => Key::BracketClose,
            msgs::KeyEvent::SLASH => Key::BackSlash, // We don't have Slash.
            msgs::KeyEvent::BACKSLASH => Key::BackSlash,
            msgs::KeyEvent::ELLIPSIS => Key::Ellipsis,
            msgs::KeyEvent::ENTER => Key::Enter,
            msgs::KeyEvent::SPACE => Key::Space,
            msgs::KeyEvent::MINUS => Key::Minus,
            msgs::KeyEvent::EQUAL => Key::Equal,
            msgs::KeyEvent::UP => Key::Up,
            msgs::KeyEvent::DOWN => Key::Down,
            msgs::KeyEvent::LEFT => Key::Left,
            msgs::KeyEvent::RIGHT => Key::Right,
            msgs::KeyEvent::SHIFT => Key::Shift,
            msgs::KeyEvent::CTRL => Key::Ctrl,
            msgs::KeyEvent::ALT => Key::Alt,
        };
        _ = dispatcher.on_input(key).await;
    }
}
