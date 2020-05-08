pub(crate) struct DateTime {
    date_time: chrono::DateTime<chrono::Local>,
}

#[cfg_attr(all(test, feature = "mocking"), mocktopus::macros::mockable)]
impl DateTime {
    pub(crate) fn now() -> Self {
        Self {
            date_time: chrono::Local::now(),
        }
    }

    pub(crate) fn format(&self, format: &str) -> String {
        self.date_time.format(format).to_string()
    }
}
