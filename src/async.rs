use error::*;
use std::sync::mpsc;
use std::thread;
use std::time;

#[derive(Debug)]
pub struct Message {
    pub id: String,
}

pub fn schedule_update(
    feature: String,
    id: String,
    interval: time::Duration,
    tx: mpsc::Sender<Message>,
) -> Result<()> {
    thread::spawn(move || loop {
        thread::sleep(interval);

        send_message(&feature, &id, &tx);
    });

    Ok(())
}

pub fn send_message(feature: &str, id: &str, tx: &mpsc::Sender<Message>) {
    let message = Message { id: id.to_owned() };

    tx.send(message)
        .wrap_error_kill(&feature, "notify thread killed");
}
