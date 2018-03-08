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

        let message = Message { id: id.clone() };
        tx.send(message)
            .wrap_error(&feature, "notify thread killed")
            .show_error()
            .unwrap();
    });

    Ok(())
}
