use crate::error::Result;
use crate::error::WrapErrorExt;

use super::ERROR_NAME;

macro_rules! compare_property {
    ( $method:ident, $property:ident ) => {
        pub(crate) fn $method(&self, compare: &'static str) -> Result<bool> {
            Ok(if let Some(interface) = self.message.$property() {
                interface.as_cstr()
                    == std::ffi::CString::new(compare)
                        .wrap_error(ERROR_NAME, "failed to create CString")?
                        .as_c_str()
            } else {
                false
            })
        }
    };
}

pub(crate) struct Message {
    message: dbus::Message,
}

impl Message {
    compare_property!(is_interface, interface);

    compare_property!(is_member, member);

    pub(crate) const fn new(message: dbus::Message) -> Self {
        Self { message }
    }

    pub(crate) fn init_method_call(
        bus: &'static str,
        path: &'_ str,
        interface: &'static str,
        member: &'static str,
    ) -> Result<Self> {
        Ok(Self {
            message: dbus::Message::new_method_call(bus, path, interface, member)
                .wrap_error(ERROR_NAME, "failed to create dbus method call message")?,
        })
    }

    #[allow(clippy::missing_const_for_fn)]
    pub(super) fn raw(self) -> dbus::Message {
        self.message
    }

    pub(crate) fn return_value<'a, T>(&'a self) -> Result<T>
    where
        T: dbus::arg::Arg + dbus::arg::Get<'a>,
    {
        self.message
            .read1::<T>()
            .wrap_error(ERROR_NAME, "failed to read return value of dbus message")
    }
}
