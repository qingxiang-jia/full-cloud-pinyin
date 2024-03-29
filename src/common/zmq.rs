use std::borrow::Cow;

use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Writer};
use zmq::Error;

use crate::{
    common::msgs::KeyEvent,
    common::msgs::{
        mod_CommandToFcitx::OneOfcommand, CommandToFcitx, CommitText, KeyEventReply,
        UpdateCandidates, UpdatePreedit,
    },
};

// Fcitx Bridge sends key events to us, so we are the server.
pub struct Server {
    ctx: zmq::Context,
    sock: zmq::Socket,
}

impl Server {
    pub fn new(bridge_addr: &str) -> Self {
        let ctx = zmq::Context::new();

        let sub = ctx
            .socket(zmq::REP)
            .expect("Failed to create a REP socket.");
        sub.bind(bridge_addr)
            .expect("Failed to bind to the key event address.");
        Server { ctx, sock: sub }
    }

    pub fn recv(&self) -> Result<KeyEvent, Error> {
        let res = self.sock.recv_msg(0);
        match res {
            Ok(data) => unsafe {
                let bytes = std::slice::from_raw_parts(data.as_ptr(), data.len());
                let mut reader = BytesReader::from_bytes(&bytes);
                let event = KeyEvent::from_reader(&mut reader, bytes)
                    .expect("Failed to decode key event message as FcitxEvent.");
                Ok(event)
            },
            Err(e) => Err(e),
        }
    }

    pub fn send(&self, accepted: bool) -> Result<(), Error> {
        let cmd = KeyEventReply { accepted };

        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);

        cmd.write_message(&mut writer)
            .expect("Failed to write message.");

        self.sock.send(out, 0)
    }
}

// We send the API calls to Fcitx Bridege, so we are the client.
pub struct Client {
    ctx: zmq::Context,
    sock: zmq::Socket,
}

impl Client {
    pub fn new(bridge_addr: &str) -> Self {
        let ctx = zmq::Context::new();

        let req = ctx
            .socket(zmq::REQ)
            .expect("Failed to create a REQ socket.");
        req.connect(bridge_addr)
            .expect("Failed to connect to the reply address.");

        Client { ctx, sock: req }
    }

    pub fn commit_text(&self, text: &str) {
        self.send_cmd(&CommandToFcitx {
            command: OneOfcommand::commit_text(CommitText {
                text: Cow::from(text),
            }),
        });
    }

    pub fn update_preedit(&self, text: &str) {
        self.send_cmd(&CommandToFcitx {
            command: OneOfcommand::update_preedit(UpdatePreedit {
                text: Cow::from(text),
            }),
        });
    }

    pub fn update_candidates(&self, words: &[String]) {
        let mut cow_words = Vec::new();
        for word in words {
            cow_words.push(Cow::from(word));
        }

        self.send_cmd(&CommandToFcitx {
            command: OneOfcommand::update_candidates(UpdateCandidates {
                candidates: cow_words,
            }),
        });
    }

    fn send_cmd(&self, cmd: &CommandToFcitx) {
        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);

        cmd.write_message(&mut writer)
            .expect("Failed to write message.");

        let snd_res = self.sock.send(out, 0);
        // .expect("Failed to send to Fcitx Bridge.");

        match snd_res {
            Ok(_) => {
                _ = self
                    .sock
                    .recv_msg(0)
                    .expect("Failed to receive or decode reply of REQ.");
            }
            Err(e) => {
                eprintln!("send_cmd: Failed to send to Fcitx Bridge. Error: {:#?}", e);
            }
        }
    }
}
