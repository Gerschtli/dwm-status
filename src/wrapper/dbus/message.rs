use super::ERROR_NAME;
use dbus;
use error::*;
use std::ffi;

macro_rules! compare_property {
    ($method:ident, $property:ident) => {
        pub fn $method(&self, compare: &'static str) -> Result<bool> {
            Ok(if let Some(interface) = self.message.$property() {
                interface.as_cstr() == ffi::CString::new(compare)
                    .wrap_error(ERROR_NAME, "failed to create CString")?
                    .as_c_str()
            } else {
                false
            })
        }
    };
}

#[derive(Debug)]
pub struct Message {
    message: dbus::Message,
}

impl Message {
    compare_property!(is_interface, interface);

    compare_property!(is_member, member);

    pub fn new(message: dbus::Message) -> Self {
        Message { message }
    }

    pub fn new_method_call<'a>(
        bus: &'static str,
        path: &'a str,
        interface: &'static str,
        member: &'static str,
    ) -> Result<Self> {
        Ok(Message {
            message: dbus::Message::new_method_call(bus, path, interface, member)
                .wrap_error(ERROR_NAME, "failed to create dbus method call message")?,
        })
    }

    pub fn raw(self) -> dbus::Message {
        self.message
    }

    pub fn return_value<'a, T>(&'a self) -> Result<T>
    where
        T: dbus::arg::Arg + dbus::arg::Get<'a>,
    {
        self.message
            .read1::<T>()
            .wrap_error(ERROR_NAME, "failed to read return value of dbus message")
    }
}
