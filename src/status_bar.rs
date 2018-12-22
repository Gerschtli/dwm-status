use communication;
use error::*;
use feature;
use settings;
use std::iter;
use std::iter::FromIterator;
use wrapper::xsetroot;

pub(crate) struct StatusBar {
    features: Vec<Box<dyn feature::Feature>>,
    string_list: Vec<String>,
    xsetroot: xsetroot::XSetRoot,
}

impl StatusBar {
    pub(crate) fn new(features: Vec<Box<dyn feature::Feature>>) -> Result<Self> {
        let string_list = Vec::from_iter(iter::repeat(String::new()).take(features.len()));

        Ok(Self {
            features,
            string_list,
            xsetroot: xsetroot::XSetRoot::new()?,
        })
    }

    pub(crate) fn update(
        &mut self,
        message: &communication::Message,
        settings: &settings::Settings,
    ) -> Result<()> {
        match message {
            communication::Message::FeatureUpdate(id) if *id < self.features.len() => {
                self.update_feature(*id, settings)?;
                self.render(settings)?;
            },
            communication::Message::FeatureUpdate(id) => {
                return Err(Error::new_custom(
                    "invalid message",
                    &format!("feature id {} does not exist", id),
                ));
            },
            communication::Message::UpdateAll => {
                if settings.general.debug {
                    println!("update all");
                }

                for id in 0..self.features.len() {
                    self.update_feature(id, settings)?;
                }
                self.render(settings)?;
            },
            _ => (),
        }

        Ok(())
    }

    fn update_feature(&mut self, id: usize, settings: &settings::Settings) -> Result<()> {
        let feature = &mut self.features[id];
        let rendered = feature.update()?.render(settings);

        if settings.general.debug {
            println!("update {}: {}", feature.name(), &rendered);
        }

        self.string_list[id] = rendered;
        Ok(())
    }

    pub(crate) fn render(&self, settings: &settings::Settings) -> Result<()> {
        let status = self.string_list.join(&settings.general.separator);

        self.xsetroot.render(status)
    }
}
