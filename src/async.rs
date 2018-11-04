use error::*;
use std::sync::mpsc;

#[derive(Debug)]
pub enum Message {
    FeatureUpdate(String),
    Kill,
    UpdateAll,
}

pub fn send_message(feature: &str, id: &str, tx: &mpsc::Sender<Message>) {
    let message = Message::FeatureUpdate(String::from(id));

    tx.send(message)
        .wrap_error_kill(feature, "notify thread killed");
}
