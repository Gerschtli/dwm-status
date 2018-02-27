use std::result::Result as StdResult;
use std::fmt;

use io::show_notification;

pub type Result<T> = StdResult<T, Error>;

pub struct Error {
    feature: String,
    description: String,
    cause: String,
}

impl Error {
    fn new<E: fmt::Debug>(feature: &str, description: &str, cause: E) -> Self {
        Error {
            feature: feature.to_owned(),
            description: description.to_owned(),
            cause: format!("{:?}", cause),
        }
    }

    pub fn show_error(self) {
        eprintln!("{:?}", self);
        show_notification(&format!("DWM-Status Error: {}", self.feature), &self.description);
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
            "Error => {}: {} ({})",
            self.feature,
            self.description,
            self.cause
        )
    }
}

pub trait ResultExt<T> {
    fn feature_error(self, feature: &str, description: &str) -> Result<T>;
}

impl<T, E: fmt::Debug> ResultExt<T> for StdResult<T, E> {
    fn feature_error(self, feature: &str, description: &str) -> Result<T> {
        self.map_err(|error| Error::new(feature, description, error))
    }
}
