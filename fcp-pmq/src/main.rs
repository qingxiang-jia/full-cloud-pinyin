use nix::mqueue::{mq_close, mq_open, mq_receive, mq_send, MQ_OFlag, MqdT};
use nix::sys::stat::Mode;

fn main() {}

fn open_send(name: &str) -> MqdT {
    mq_open(
        name,
        MQ_OFlag::O_CREAT | MQ_OFlag::O_WRONLY,
        Mode::S_IWUSR | Mode::S_IRUSR | Mode::S_IRGRP | Mode::S_IROTH,
        None,
    )
    .expect("Failed to open queue to send.")
}

fn open_recv(name: &str) -> MqdT {
    mq_open(
        name,
        MQ_OFlag::O_CREAT | MQ_OFlag::O_RDONLY,
        Mode::S_IWUSR | Mode::S_IRUSR | Mode::S_IRGRP | Mode::S_IROTH,
        None,
    )
    .expect("Failed to open queue to receive.")
}

fn send(q: &MqdT, payload: &[u8]) {
    mq_send(q, payload, 1);
}

fn recv(q: &MqdT, payload: &mut [u8]) {
    let mut prio = 1;
    mq_receive(q, payload, &mut prio);
}

fn close(q: MqdT) {
    mq_close(q).expect("Faild to close the queue.");
}
