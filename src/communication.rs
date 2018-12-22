use error::*;
use std::sync::mpsc;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Message {
    FeatureUpdate(usize),
    Kill,
    UpdateAll,
}

pub(crate) fn send_message(feature: &str, id: usize, tx: &mpsc::Sender<Message>) -> Result<()> {
    let message = Message::FeatureUpdate(id);

    tx.send(message).wrap_error(feature, "notify thread killed")
}
