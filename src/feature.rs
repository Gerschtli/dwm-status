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

pub trait Renderable {
    fn render(&self, &settings::Settings) -> String;
}

pub trait Feature {
    fn id(&self) -> uuid::Uuid;

    fn init_notifier(&self) -> Result<()>;

    fn name(&self) -> &str;

    fn update(&mut self) -> Result<Box<dyn Renderable>>;
}

pub trait FeatureConfig: Feature {
    type Settings;

    fn new(uuid::Uuid, mpsc::Sender<async::Message>, Self::Settings) -> Result<Self>
    where
        Self: Sized;
}
