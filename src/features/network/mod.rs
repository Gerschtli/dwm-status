mod config;
mod data;

pub(crate) use self::config::ConfigEntry;
pub(self) use self::config::RenderConfig;
pub(self) use self::config::UpdateConfig;
pub(self) use self::data::Data;

pub(super) const FEATURE_NAME: &str = "network";
pub(self) const PLACEHOLDER_ESSID: &str = "{ESSID}";
pub(self) const PLACEHOLDER_IPV4: &str = "{IPv4}";
pub(self) const PLACEHOLDER_IPV6: &str = "{IPv6}";
