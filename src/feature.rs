use async;
use error::*;
use std::sync::mpsc;

pub trait Feature {
    fn id(&self) -> &str;

    fn init_notifier(&self) -> Result<()>;

    fn render(&self) -> String;

    fn update(&mut self) -> Result<()>;
}

pub trait FeatureConfig: Feature {
    fn new(id: String, tx: mpsc::Sender<async::Message>) -> Result<Self>
    where
        Self: Sized;
}
