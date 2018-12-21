use feature;
use settings;
use utils::icon_by_percentage;

#[derive(Debug)]
pub(super) enum Data {
    Mute,
    Volume(u32),
}

impl feature::Renderable for Data {
    fn render(&self, settings: &settings::Settings) -> String {
        match *self {
            Data::Mute => settings.audio.mute.clone(),
            Data::Volume(volume) => {
                let mut rendered = settings
                    .audio
                    .template
                    .replace("{VOL}", &volume.to_string());

                if let Some(icon) = icon_by_percentage(&settings.audio.icons, volume) {
                    rendered = rendered.replace("{ICO}", icon);
                }

                rendered
            },
        }
    }
}
