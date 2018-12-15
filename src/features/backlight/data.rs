use feature;
use settings;
use utils::icon_by_percentage;

#[derive(Debug, Default)]
pub(super) struct BacklightData(pub(super) u16);

impl feature::Renderable for BacklightData {
    fn render(&self, settings: &settings::Settings) -> String {
        let mut rendered = settings
            .backlight
            .template
            .replace("{BL}", &format!("{}", self.0));

        if let Some(icon) = icon_by_percentage(&settings.backlight.icons, self.0) {
            rendered = rendered.replace("{ICO}", icon);
        }

        rendered
    }
}
