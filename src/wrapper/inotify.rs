pub(crate) use inotify::WatchMask;

use crate::error::Result;
use crate::error::WrapErrorExt;

const ERROR_NAME: &str = "inotify";

pub(crate) struct Inotify {
    inotify: inotify::Inotify,
}

impl Inotify {
    pub(crate) fn init() -> Result<Self> {
        let inotify = inotify::Inotify::init().wrap_error(ERROR_NAME, "failed to start inotify")?;

        Ok(Self { inotify })
    }

    pub(crate) fn add_watch(&mut self, path: &str, mask: WatchMask) -> Result<()> {
        self.inotify
            .add_watch(path, mask)
            .wrap_error(ERROR_NAME, format!("failed to watch '{}'", path))?;

        Ok(())
    }

    pub(crate) fn listen_for_any_events<F>(&mut self, handler: F) -> Result<()>
    where
        F: Fn() -> Result<()>,
    {
        let mut buffer = [0; 1024];
        loop {
            self.inotify
                .read_events_blocking(&mut buffer)
                .wrap_error(ERROR_NAME, "error while reading inotify events")?;

            handler()?;
        }
    }
}
