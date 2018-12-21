use error::*;
use settings;
use wrapper::thread;

pub(crate) trait Renderable {
    fn render(&self, _: &settings::Settings) -> String;
}

pub(crate) trait Feature {
    fn init_notifier(&mut self) -> Result<()>;

    fn name(&self) -> &'static str;

    fn update(&mut self) -> Result<Box<dyn Renderable>>;
}

pub(crate) trait Updatable {
    type Data: Renderable + 'static;

    fn update(&mut self) -> Result<Self::Data>;
}

pub(crate) struct Composer<N, U>
where
    N: thread::Runnable + Send + 'static,
    U: Updatable,
{
    name: &'static str,
    notifier: Option<N>,
    updater: U,
}

impl<N, U> Composer<N, U>
where
    N: thread::Runnable + Send + 'static,
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
    N: thread::Runnable + Send + 'static,
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

    fn update(&mut self) -> Result<Box<dyn Renderable>> {
        Ok(Box::new(self.updater.update()?))
    }
}
