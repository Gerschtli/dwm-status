pub(crate) mod data;
pub(crate) mod message;

use dbus;
use error::*;

pub(crate) use self::data::Match;
pub(crate) use self::message::Message;
pub(crate) use dbus::Path;

const ERROR_NAME: &str = "dbus";

pub(crate) struct Connection {
    connection: dbus::Connection,
}

impl Connection {
    pub(crate) fn new() -> Result<Self> {
        let connection = dbus::Connection::get_private(dbus::BusType::System)
            .wrap_error(ERROR_NAME, "failed to connect to dbus")?;

        Ok(Connection { connection })
    }

    pub(crate) fn add_match(&self, match_: Match) -> Result<()> {
        self.connection
            .add_match(&match_.build())
            .wrap_error(ERROR_NAME, "failed to add match")
    }

    pub(crate) fn listen_for_signals<T>(&self, mut handle_signal: T) -> Result<()>
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

    pub(crate) fn remove_match(&self, match_: Match) -> Result<()> {
        self.connection
            .remove_match(&match_.build())
            .wrap_error(ERROR_NAME, "failed to remove match")
    }

    pub(crate) fn send_message(&self, message: Message) -> Result<Message> {
        Ok(Message::new(
            self.connection
                .send_with_reply_and_block(message.raw(), 2000) // 2 seconds timeout
                .wrap_error(ERROR_NAME, "failed to send message")?,
        ))
    }
}
