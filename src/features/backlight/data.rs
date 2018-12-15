use feature;
use settings;
use utils::icon_by_float;

#[derive(Debug, Default)]
pub struct BacklightData(pub f32);

impl feature::Renderable for BacklightData {
    fn render(&self, settings: &settings::Settings) -> String {
        let mut rendered = settings
            .backlight
            .template
            .replace("{BL}", &format!("{:.0}", self.0 * 100.));

        if let Some(icon) = icon_by_float(&settings.backlight.icons, self.0) {
            rendered = rendered.replace("{ICO}", icon);
        }

        rendered
    }
}
