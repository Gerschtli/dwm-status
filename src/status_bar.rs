use async;
use error::*;
use feature;
use settings;
use std::collections::HashMap;
use uuid;
use wrapper::xsetroot;

pub(crate) struct StatusBar {
    feature_map: HashMap<uuid::Uuid, Box<dyn feature::Feature>>,
    order: Vec<uuid::Uuid>,
    string_map: HashMap<uuid::Uuid, String>,
    xsetroot: xsetroot::XSetRoot,
}

impl StatusBar {
    pub(crate) fn new(features: Vec<Box<dyn feature::Feature>>) -> Result<Self> {
        let order: Vec<_> = features.iter().map(|feature| feature.id()).collect();

        let string_map: HashMap<_, _> = features
            .iter()
            .map(|feature| (feature.id(), String::new()))
            .collect();

        let feature_map: HashMap<_, _> = features
            .into_iter()
            .map(|feature| (feature.id(), feature))
            .collect();

        Ok(StatusBar {
            feature_map,
            order,
            string_map,
            xsetroot: xsetroot::XSetRoot::new()?,
        })
    }

    pub(crate) fn update(
        &mut self,
        message: &async::Message,
        settings: &settings::Settings,
    ) -> Result<()> {
        match message {
            async::Message::FeatureUpdate(id) if self.feature_map.contains_key(id) => {
                self.update_feature(*id, settings)?;
                self.render(settings)?;
            },
            async::Message::FeatureUpdate(id) => {
                return Err(Error::new_custom(
                    "invalid message",
                    &format!("message id {} does not exist", id),
                ));
            },
            async::Message::UpdateAll => {
                if settings.debug {
                    println!("update all");
                }

                for id in self.order.clone() {
                    self.update_feature(id, settings)?;
                }
                self.render(settings)?;
            },
            _ => (),
        }

        Ok(())
    }

    fn update_feature(&mut self, id: uuid::Uuid, settings: &settings::Settings) -> Result<()> {
        let feature = self.feature_map.get_mut(&id).unwrap();
        let rendered = feature.update()?.render(settings);

        if settings.debug {
            println!("update {}: {}", feature.name(), &rendered);
        }

        self.string_map.insert(id, rendered);
        Ok(())
    }

    pub(crate) fn render(&self, settings: &settings::Settings) -> Result<()> {
        let status = self
            .order
            .iter()
            .map(|id| &self.string_map[id][..])
            .collect::<Vec<_>>()
            .join(&settings.separator);

        self.xsetroot.render(status)
    }
}
