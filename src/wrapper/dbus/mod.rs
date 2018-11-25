pub mod data;
pub mod message;

use dbus;
use error::*;

pub use self::data::Match;
pub use self::message::Message;
pub use dbus::Path;

pub const ERROR_NAME: &str = "dbus";

pub struct Connection {
    connection: dbus::Connection,
}

impl Connection {
    pub fn new() -> Result<Self> {
        let connection = dbus::Connection::get_private(dbus::BusType::System)
            .wrap_error(ERROR_NAME, "failed to connect to dbus")?;

        Ok(Connection { connection })
    }

    pub fn add_match(&self, match_: Match) -> Result<()> {
        self.connection
            .add_match(&match_.build())
            .wrap_error(ERROR_NAME, "failed to add match")
    }

    pub fn listen_for_signals<T>(&self, mut handle_signal: T) -> Result<()>
    where
        T: FnMut(Message) -> Result<()>,
    {
        // 300_000 seconds timeout before sending ConnectionItem::Nothing
        for item in self.connection.iter(300_000) {
            if let dbus::ConnectionItem::Signal(signal) = item {
                handle_signal(Message::new(signal))?;
            }
        }

        Ok(())
    }

    pub fn remove_match(&self, match_: Match) -> Result<()> {
        self.connection
            .remove_match(&match_.build())
            .wrap_error(ERROR_NAME, "failed to remove match")
    }

    pub fn send_message(&self, message: Message) -> Result<Message> {
        Ok(Message::new(
            self.connection
                .send_with_reply_and_block(message.raw(), 2000) // 2 seconds timeout
                .wrap_error(ERROR_NAME, "failed to send message")?,
        ))
    }
}
