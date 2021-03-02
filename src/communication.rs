use crate::error::Result;
use crate::wrapper::channel;

#[derive(Clone, Copy, Debug)]
pub(crate) enum Message {
    FeatureUpdate(usize),
    Kill,
    UpdateAll,
}

pub(crate) fn send_message(id: usize, sender: &channel::Sender<Message>) -> Result<()> {
    let message = Message::FeatureUpdate(id);

    sender.send(message)
}
