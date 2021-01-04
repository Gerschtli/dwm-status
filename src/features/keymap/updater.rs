use super::Data;
use crate::error::*;
use crate::feature;
use xcb;
use xcb::xkb;

pub(super) struct Updater {
    data: Data,
}

impl Updater {
    pub(super) const fn new(data: Data) -> Self {
        Self { data }
    }
}

impl feature::Updatable for Updater {
    fn renderable(&self) -> &dyn feature::Renderable {
        &self.data
    }

    fn update(&mut self) -> Result<()> {
        if let Ok((conn, _)) = xcb::Connection::connect(None) {
            let cookie = xkb::use_extension(&conn, 1, 0);
            match cookie.get_reply() {
                Ok(r) => {
                    if !r.supported() {
                        return Err(Error::new_custom("keymap", "xkb is not supported"));
                    }
                },
                Err(_) => return Err(Error::new_custom("keymap", "invalid reply from xkb")),
            }
            let state = xkb::get_state(&conn, xkb::ID_USE_CORE_KBD as u16);
            match state.get_reply() {
                Ok(r) => {
                    self.data.update(r.group());
                },
                Err(_) => return Err(Error::new_custom("keymap", "invalid reply from xkb")),
            }
            Ok(())
        } else {
            Err(Error::new_custom("keymap", "failed to update keymap state"))
        }
    }
}
