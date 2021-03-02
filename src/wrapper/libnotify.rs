pub(crate) use libnotify::Urgency;

use crate::error::Result;
use crate::error::WrapErrorExt;

const ERROR_NAME: &str = "libnotify";

pub(crate) struct LibNotify;

impl LibNotify {
    pub(crate) fn init() -> Result<Self> {
        libnotify::init("dwm-status").wrap_error(ERROR_NAME, "init failed")?;

        Ok(Self {})
    }

    pub(crate) fn send_notification(
        &self,
        summary: &str,
        body: Option<&str>,
        urgency: Urgency,
    ) -> Result<()> {
        let notification = libnotify::Notification::new(summary, body, None);
        notification.set_urgency(urgency);
        notification
            .show()
            .wrap_error(ERROR_NAME, "send notification failed")
    }
}

impl Drop for LibNotify {
    fn drop(&mut self) {
        libnotify::uninit();
    }
}
