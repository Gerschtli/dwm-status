use async;
use error::*;
use std::sync::mpsc;

macro_rules! feature_default {
    () => {
        fn id(&self) -> &str {
            &self.id
        }

        fn name(&self) -> &str {
            FEATURE_NAME
        }

        fn render(&self) -> String {
            format!("{}", self.data)
        }
    }
}

pub trait Feature {
    fn id(&self) -> &str;

    fn init_notifier(&self) -> Result<()>;

    fn name(&self) -> &str;

    fn render(&self) -> String;

    fn update(&mut self) -> Result<()>;
}

pub trait FeatureConfig: Feature {
    fn new(id: String, tx: mpsc::Sender<async::Message>) -> Result<Self>
    where
        Self: Sized;
}
