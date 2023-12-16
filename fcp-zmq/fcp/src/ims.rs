use std::borrow::Cow;

use quick_protobuf::{BytesReader, MessageRead, MessageWrite, Writer};

use crate::{
    ims_recv::{
        mod_CommandToFcitx::OneOfcommand, CommandToFcitx, CommitText, LookupTable,
        UpdateLookuptable, UpdatePreedit,
    },
    ims_send::FcitxEvent,
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

    pub fn recv(&self) -> FcitxEvent {
        let data = self
            .sock
            .recv_msg(0)
            .expect("Failed to receive published message.");
        unsafe {
            let bytes = std::slice::from_raw_parts(data.as_ptr(), data.len());
            let mut reader = BytesReader::from_bytes(&bytes);
            let fe = FcitxEvent::from_reader(&mut reader, bytes)
                .expect("Failed to decode published message as FcitxEvent.");
            return fe;
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

    pub fn commit_text(&self, text: &str) {
        let cmd = CommitText {
            text: Cow::from(text),
        };
        let cmd_container = CommandToFcitx {
            command: OneOfcommand::commit_text(cmd),
        };

        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);

        cmd_container
            .write_message(&mut writer)
            .expect("Failed to write message for CommitText.");

        self.sock.send(out, 0).expect("Failed to send to IMS.");
        _ = self
            .sock
            .recv_msg(0)
            .expect("Failed to receive reply of REQ.");
    }

    pub fn update_preedit(&self, text: &str) {
        let cmd = UpdatePreedit {
            text: Cow::from(text),
        };
        let cmd_container = CommandToFcitx {
            command: OneOfcommand::update_preedit(cmd),
        };

        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);

        cmd_container
            .write_message(&mut writer)
            .expect("Failed to write message for UpdatePreedit.");

        self.sock.send(out, 0).expect("Failed to send to IMS.");
        _ = self
            .sock
            .recv_msg(0)
            .expect("Failed to receive reply of REQ.");
    }

    pub fn update_lookuptable(&self, words: &[String]) {
        let mut cow_vec = Vec::new();
        for word in words {
            cow_vec.push(Cow::from(word));
        }
        let lt = LookupTable {
            candidates: cow_vec,
        };
        let cmd = UpdateLookuptable { lt: Some(lt) };
        let cmd_container = CommandToFcitx {
            command: OneOfcommand::update_lt(cmd),
        };

        let mut out = Vec::new();
        let mut writer = Writer::new(&mut out);

        cmd_container
            .write_message(&mut writer)
            .expect("Failed to write message for UpdateLookuptable.");

        self.sock.send(out, 0).expect("Failed to send to IMS.");
        _ = self
            .sock
            .recv_msg(0)
            .expect("Failed to receive reply of REQ.");
    }
}
