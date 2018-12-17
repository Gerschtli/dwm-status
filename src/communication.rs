use error::*;
use std::sync::mpsc;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Message {
    FeatureUpdate(usize),
    Kill,
    UpdateAll,
}

pub(crate) fn send_message(feature: &str, id: usize, tx: &mpsc::Sender<Message>) {
    let message = Message::FeatureUpdate(id);

    tx.send(message)
        .wrap_error_kill(feature, "notify thread killed");
}
