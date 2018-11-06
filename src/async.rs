use error::*;
use std::sync::mpsc;
use uuid;

#[derive(Clone, Copy, Debug)]
pub enum Message {
    FeatureUpdate(uuid::Uuid),
    Kill,
    UpdateAll,
}

pub fn send_message(feature: &str, id: uuid::Uuid, tx: &mpsc::Sender<Message>) {
    let message = Message::FeatureUpdate(id);

    tx.send(message)
        .wrap_error_kill(feature, "notify thread killed");
}
