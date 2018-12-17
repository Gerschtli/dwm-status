use communication;
use error::*;
use settings;
use std::sync::mpsc;

macro_rules! feature_default {
    () => {
        fn id(&self) -> usize {
            self.id
        }

        fn name(&self) -> &str {
            FEATURE_NAME
        }
    }
}

pub(crate) trait Renderable {
    fn render(&self, _: &settings::Settings) -> String;
}

pub(crate) trait Feature {
    fn id(&self) -> usize;

    fn init_notifier(&self) -> Result<()>;

    fn name(&self) -> &str;

    fn update(&mut self) -> Result<Box<dyn Renderable>>;
}

pub(crate) trait FeatureConfig: Feature {
    type Settings;

    fn new(_: usize, _: mpsc::Sender<communication::Message>, _: Self::Settings) -> Result<Self>
    where
        Self: Sized;
}
