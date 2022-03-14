use std::thread;
use std::time;

use crate::error::Result;
use crate::error::ResultExt;
use crate::error::WrapErrorExt;

pub(crate) trait Runnable: Send + 'static {
    fn run(&self) -> Result<()>;
}

pub(crate) struct Thread<R> {
    name: &'static str,
    runnable: R,
}

impl<R> Thread<R>
where
    R: Runnable,
{
    #[allow(clippy::missing_const_for_fn)] // not supported by stable
    pub(crate) fn new(name: &'static str, runnable: R) -> Self {
        Self { name, runnable }
    }

    pub(crate) fn run(self) -> Result<()> {
        thread::Builder::new()
            .name(self.name.to_owned())
            .spawn(move || {
                loop {
                    self.runnable.run().show_error_and_ignore();
                    sleep_secs(2);
                }
            })
            .wrap_error("thread start", "failed to create thread")?;

        Ok(())
    }
}

pub(crate) fn sleep_secs(seconds: u64) {
    thread::sleep(time::Duration::from_secs(seconds));
}

pub(crate) fn sleep_prevent_spam() {
    thread::sleep(time::Duration::from_millis(100));
}
