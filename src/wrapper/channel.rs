use std::sync::mpsc;

use crate::error::Result;
use crate::error::WrapErrorExt;

pub(crate) fn create<M: Clone>() -> (Sender<M>, Receiver<M>) {
    let (tx, rx) = mpsc::channel();

    (Sender { sender: tx }, Receiver { receiver: rx })
}

pub(crate) struct Receiver<M: Clone> {
    receiver: mpsc::Receiver<M>,
}

impl<M: Clone> Receiver<M> {
    pub(crate) fn read_blocking(&self) -> Result<M> {
        self.receiver
            .recv()
            .wrap_error("channel receiver", "read blocking failed")
    }
}

#[derive(Clone)]
pub(crate) struct Sender<M> {
    sender: mpsc::Sender<M>,
}

impl<M> Sender<M> {
    pub(crate) fn send(&self, message: M) -> Result<()> {
        self.sender
            .send(message)
            .wrap_error("channel sender", "notify thread killed")
    }
}
