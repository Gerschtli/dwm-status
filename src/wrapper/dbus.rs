use dbus::ffidisp::BusType;
use dbus::ffidisp::Connection as DbusConnection;
use dbus::ffidisp::ConnectionItem;
pub(crate) use dbus::Path;

use crate::error::Result;
use crate::error::WrapErrorExt;

pub(crate) use self::data::Match;
pub(crate) use self::message::Message;

pub(crate) mod data;
pub(crate) mod message;

const ERROR_NAME: &str = "dbus";

pub(crate) struct Connection {
    connection: DbusConnection,
}

impl Connection {
    pub(crate) fn init() -> Result<Self> {
        let connection = DbusConnection::get_private(BusType::System)
            .wrap_error(ERROR_NAME, "failed to connect to dbus")?;

        Ok(Self { connection })
    }

    pub(crate) fn add_match(&self, match_: Match<'_>) -> Result<()> {
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
            if let ConnectionItem::Signal(signal) = item {
                handle_signal(Message::new(signal))?;
            }
        }

        Ok(())
    }

    pub(crate) fn remove_match(&self, match_: Match<'_>) -> Result<()> {
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
