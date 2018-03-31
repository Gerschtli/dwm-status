use error::*;
use std::sync::mpsc;

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
