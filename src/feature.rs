use crate::error::*;
use crate::wrapper::thread;

pub(crate) trait Renderable {
    fn render(&self) -> &str;
}

pub(crate) trait Updatable {
    fn renderable(&self) -> &dyn Renderable;

    fn update(&mut self) -> Result<()>;
}

pub(crate) trait Feature: Updatable {
    fn init_notifier(&mut self) -> Result<()>;

    fn name(&self) -> &'static str;
}

pub(crate) struct Composer<N, U>
where
    N: thread::Runnable,
    U: Updatable,
{
    name: &'static str,
    notifier: Option<N>,
    updater: U,
}

impl<N, U> Composer<N, U>
where
    N: thread::Runnable,
    U: Updatable,
{
    pub(crate) fn new(name: &'static str, notifier: N, updater: U) -> Self {
        Self {
            name,
            notifier: Some(notifier),
            updater,
        }
    }
}

impl<N, U> Feature for Composer<N, U>
where
    N: thread::Runnable,
    U: Updatable,
{
    fn init_notifier(&mut self) -> Result<()> {
        if let Some(notifier) = self.notifier.take() {
            let thread = thread::Thread::new(self.name, notifier);
            thread.run()
        } else {
            Err(Error::new_custom("feature", "can not start notifier twice"))
        }
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

impl<N, U> Updatable for Composer<N, U>
where
    N: thread::Runnable,
    U: Updatable,
{
    fn renderable(&self) -> &dyn Renderable {
        self.updater.renderable()
    }

    fn update(&mut self) -> Result<()> {
        self.updater.update()
    }
}
