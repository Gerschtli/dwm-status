use communication;
use error::*;
use feature;
use settings;
use wrapper::xsetroot;

pub(crate) struct StatusBar {
    features: Vec<Box<dyn feature::Feature>>,
    xsetroot: xsetroot::XSetRoot,
}

impl StatusBar {
    pub(crate) fn new(features: Vec<Box<dyn feature::Feature>>) -> Result<Self> {
        Ok(Self {
            features,
            xsetroot: xsetroot::XSetRoot::new()?,
        })
    }

    pub(crate) fn update(
        &mut self,
        message: &communication::Message,
        settings: &settings::General,
    ) -> Result<()> {
        match message {
            communication::Message::FeatureUpdate(id) if *id < self.features.len() => {
                info!("Update feature {}", self.features[*id].name());

                self.features[*id].update()?;
                self.render(settings)?;
            },
            communication::Message::FeatureUpdate(id) => {
                return Err(Error::new_custom(
                    "invalid message",
                    &format!("feature id {} does not exist", id),
                ));
            },
            communication::Message::UpdateAll => {
                info!("Update all features");

                for id in 0..self.features.len() {
                    self.features[id].update()?;
                }
                self.render(settings)?;
            },
            _ => (),
        }

        Ok(())
    }

    pub(crate) fn render(&self, settings: &settings::General) -> Result<()> {
        let status = self
            .features
            .iter()
            .map(|f| f.renderable().render())
            .collect::<Vec<_>>()
            .join(&settings.separator);

        self.xsetroot.render(status)
    }
}
