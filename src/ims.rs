use std::borrow::Cow;

use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Writer};

use crate::{
    msgs::KeyEvent,
    msgs::{
        mod_CommandToFcitx::OneOfcommand, CommandToFcitx, CommitText, UpdateCandidates,
        UpdatePreedit, UpdateSessionStatus,
    },
};

pub struct Sub {
    ctx: zmq::Context,
    sock: zmq::Socket,
}

impl Sub {
    pub fn new(ims_addr: &str) -> Self {
        let ctx = zmq::Context::new();

        let sub = ctx
            .socket(zmq::SUB)
            .expect("Failed to create a SUB socket.");
        sub.connect(ims_addr)
            .expect("Failed to connect to the publisher address.");
        sub.set_subscribe(b"").expect("Failed to subscribe to any.");

        Sub { ctx, sock: sub }
    }

    pub fn recv(&self) -> KeyEvent {
        let data = self
            .sock
            .recv_msg(0)
            .expect("Failed to receive published message.");
        unsafe {
            let bytes = std::slice::from_raw_parts(data.as_ptr(), data.len());
            let mut reader = BytesReader::from_bytes(&bytes);
            let event = KeyEvent::from_reader(&mut reader, bytes)
                .expect("Failed to decode published message as FcitxEvent.");
            return event;
        }
    }
}

// Because Fcitx5 is not thread safe, so any call other than new() needs to be wrapped in a Mutex.
pub struct Req {
    ctx: zmq::Context,
    sock: zmq::Socket,
}

impl Req {
    pub fn new(ims_addr: &str) -> Self {
        let ctx = zmq::Context::new();

        let req = ctx
            .socket(zmq::REQ)
            .expect("Failed to create a REQ socket.");
        req.connect(ims_addr)
            .expect("Failed to connect to the reply address.");

        Req { ctx, sock: req }
    }

    pub fn update_session_status(&self, in_session: bool) {
        let cmd = UpdateSessionStatus { in_session };
        let cmd_container = CommandToFcitx {
            command: OneOfcommand::update_session_status(cmd),
        };

        self.send_cmd(&cmd_container);
    }

    pub fn commit_text(&self, text: &str) {
        let cmd = CommitText {
            text: Cow::from(text),
        };
        let cmd_container = CommandToFcitx {
            command: OneOfcommand::commit_text(cmd),
        };

        self.send_cmd(&cmd_container);
    }

    pub fn update_preedit(&self, text: &str) {
        let cmd = UpdatePreedit {
            text: Cow::from(text),
        };
        let cmd_container = CommandToFcitx {
            command: OneOfcommand::update_preedit(cmd),
        };

        self.send_cmd(&cmd_container);
    }

    pub fn update_candidates(&self, words: &[String]) {
        let mut cow_words = Vec::new();
        for word in words {
            cow_words.push(Cow::from(word));
        }
        let cmd = UpdateCandidates {
            candidates: cow_words,
        };
        let cmd_container = CommandToFcitx {
            command: OneOfcommand::update_candidates(cmd),
        };

        self.send_cmd(&cmd_container);
    }

    fn send_cmd(&self, cmd: &CommandToFcitx) {
        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);

        cmd.write_message(&mut writer)
            .expect("Failed to write message.");

        self.sock.send(out, 0).expect("Failed to send to IMS.");
        _ = self
            .sock
            .recv_msg(0)
            .expect("Failed to receive reply of REQ.");
    }
}
