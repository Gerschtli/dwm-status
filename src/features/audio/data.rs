use feature;
use settings;
use utils::icon_by_percentage;

#[derive(Debug)]
pub(super) enum AudioData {
    Mute,
    Volume(u16),
}

impl feature::Renderable for AudioData {
    fn render(&self, settings: &settings::Settings) -> String {
        match *self {
            AudioData::Mute => settings.audio.mute.clone(),
            AudioData::Volume(volume) => {
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
