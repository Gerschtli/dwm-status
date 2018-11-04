pub mod data;

use dbus_lib;
use error::*;
use std::ffi;

const ERROR_NAME: &str = "dbus";

macro_rules! compare_property {
    ($method:ident, $property:ident, $data:ident) => {
        pub fn $method(&self, compare: ::dbus::data::$data) -> Result<bool> {
            Ok(if let Some(interface) = self.message.$property() {
                interface.as_cstr() == ffi::CString::new(compare.value())
                    .wrap_error(ERROR_NAME, "failed to create CString")?
                    .as_c_str()
            } else {
                false
            })
        }
    };
}

pub struct Message {
    message: dbus_lib::Message,
}

impl Message {
    compare_property!(is_interface, interface, Interface);

    compare_property!(is_member, member, Member);

    pub fn new(message: dbus_lib::Message) -> Self {
        Message { message }
    }

    pub fn new_method_call<'a>(
        bus: &data::Interface,
        path: &'a str,
        interface: &data::Interface,
        member: &data::Member,
    ) -> Result<Self> {
        Ok(Message {
            message: dbus_lib::Message::new_method_call(
                bus.value(),
                path,
                interface.value(),
                member.value(),
            )
            .wrap_error(ERROR_NAME, "failed to create dbus method call message")?,
        })
    }

    fn raw(self) -> dbus_lib::Message {
        self.message
    }

    pub fn return_value<'a, T>(&'a self) -> Result<T>
    where
        T: dbus_lib::arg::Arg + dbus_lib::arg::Get<'a>,
    {
        self.message
            .read1::<T>()
            .wrap_error(ERROR_NAME, "failed to read return value of dbus message")
    }
}

pub struct Connection {
    connection: dbus_lib::Connection,
}

impl Connection {
    pub fn new() -> Result<Self> {
        let connection = dbus_lib::Connection::get_private(dbus_lib::BusType::System)
            .wrap_error(ERROR_NAME, "failed to connect to dbus")?;

        Ok(Connection { connection })
    }

    pub fn add_match(&self, match_: data::Match) -> Result<()> {
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
            if let dbus_lib::ConnectionItem::Signal(signal) = item {
                handle_signal(Message::new(signal))?;
            }
        }

        Ok(())
    }

    pub fn remove_match(&self, match_: data::Match) -> Result<()> {
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
