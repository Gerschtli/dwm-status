use async;
use error::*;
use settings;
use std::sync::mpsc;
use uuid;

macro_rules! feature_default {
    () => {
        fn id(&self) -> ::uuid::Uuid {
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
    fn id(&self) -> uuid::Uuid;

    fn init_notifier(&self) -> Result<()>;

    fn name(&self) -> &str;

    fn update(&mut self) -> Result<Box<dyn Renderable>>;
}

pub(crate) trait FeatureConfig: Feature {
    type Settings;

    fn new(_: uuid::Uuid, _: mpsc::Sender<async::Message>, _: Self::Settings) -> Result<Self>
    where
        Self: Sized;
}
