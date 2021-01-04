use crate::communication;
use crate::error::*;
use crate::wrapper::channel;
use crate::wrapper::thread;
use xcb;
use xcb::xkb;

pub(super) struct Notifier {
    id: usize,
    sender: channel::Sender<communication::Message>,
}

impl Notifier {
    pub(super) const fn new(id: usize, sender: channel::Sender<communication::Message>) -> Self {
        Self { id, sender }
    }
}

impl thread::Runnable for Notifier {
    fn run(&self) -> Result<()> {
        // Almost verbatim copy of the code from https://github.com/myfreeweb/unixbar
        if let Ok((conn, _)) = xcb::Connection::connect(None) {
            {
                let cookie = xkb::use_extension(&conn, 1, 0);
                match cookie.get_reply() {
                    Ok(r) => {
                        if !r.supported() {
                            return Err(Error::new_custom("keymap", "xkb is not supported"));
                        }
                    },
                    Err(_) => return Err(Error::new_custom("keymap", "invalid reply from xkb")),
                }
            }
            {
                let map_parts = xcb::xkb::MAP_PART_MODIFIER_MAP;
                let events = xcb::xkb::EVENT_TYPE_STATE_NOTIFY;
                let cookie = xkb::select_events_checked(
                    &conn,
                    xkb::ID_USE_CORE_KBD as u16,
                    events as u16,
                    0,
                    events as u16,
                    map_parts as u16,
                    map_parts as u16,
                    None,
                );
                let _ = cookie.request_check();
            }
            loop {
                let event = conn.wait_for_event();
                match event {
                    None => {
                        break Err(Error::new_custom("keymap", "break"));
                    },
                    Some(event) => {
                        let evt: &xkb::StateNotifyEvent = unsafe { xcb::cast_event(&event) };
                        if evt.changed() as u32 & xcb::xkb::STATE_PART_GROUP_STATE != 0 {
                            communication::send_message(self.id, &self.sender)?;
                        }
                    },
                }
            }
        } else {
            return Err(Error::new_custom("keymap", "cannot connect to x server"));
        }
    }
}
