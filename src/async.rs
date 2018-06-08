use error::*;
use std::sync::mpsc;
use std::thread;
use std::time;

#[derive(Debug)]
pub struct Message {
    pub id: String,
}

pub fn send_message(feature: &str, id: &str, tx: &mpsc::Sender<Message>) {
    let message = Message {
        id: String::from(id),
    };

    tx.send(message)
        .wrap_error_kill(feature, "notify thread killed");
}

pub fn send_message_interval(
    feature: &'static str,
    id: String,
    tx: mpsc::Sender<Message>,
    interval: u64,
) {
    thread::spawn(move || loop {
        thread::sleep(time::Duration::from_secs(interval));

        send_message(feature, &id, &tx);
    });
}
