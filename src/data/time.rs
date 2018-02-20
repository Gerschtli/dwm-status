use chrono::{DateTime, Local};
use std::fmt;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;

use super::{Feature, Message};

#[derive(Debug)]
pub struct Time(DateTime<Local>);

impl fmt::Display for Time {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.format("%Y-%m-%d %H:%M"))
    }
}

impl Feature for Time {
    fn init() -> Self {
        Time(Local::now())
    }

    fn wait_for_update(tx: &Sender<Message>) {
        loop {
            thread::sleep(Duration::from_secs(60));

            let message = Message::Time(Self::init());
            tx.send(message).unwrap();
        }
    }
}
