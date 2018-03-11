use io;
use libnotify;
use std::fmt;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

pub struct Error {
    name: String,
    description: String,
    cause: Option<String>,
}

impl Error {
    fn new<E: fmt::Debug>(name: &str, description: &str, cause: E) -> Self {
        Error {
            name: name.to_owned(),
            description: description.to_owned(),
            cause: Some(format!("{:?}", cause)),
        }
    }

    pub fn new_custom(name: &str, description: &str) -> Self {
        Error {
            name: name.to_owned(),
            description: description.to_owned(),
            cause: None,
        }
    }

    pub fn show_error(self) {
        eprintln!("{:?}", self);

        io::show_notification(
            &format!("dwm-status: {}", self.name),
            &self.description,
            libnotify::Urgency::Critical,
        );
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error => {}: {}{}",
            self.name,
            self.description,
            match self.cause {
                Some(ref cause) => format!(" ({})", cause),
                None => "".to_owned(),
            }
        )
    }
}

pub trait WrapErrorExt<T> {
    fn wrap_error(self, name: &str, description: &str) -> Result<T>;

    fn wrap_error_kill(self, name: &str, description: &str) -> T
    where
        Self: Sized,
    {
        self.wrap_error(&name, &description).show_error().unwrap()
    }
}

impl<T, E: fmt::Debug> WrapErrorExt<T> for StdResult<T, E> {
    fn wrap_error(self, name: &str, description: &str) -> Result<T> {
        self.map_err(|error| Error::new(name, description, error))
    }
}

impl<T> WrapErrorExt<T> for Option<T> {
    fn wrap_error(self, name: &str, description: &str) -> Result<T> {
        self.ok_or_else(|| Error::new_custom(name, description))
    }
}

pub trait ResultExt<T> {
    fn show_error(self) -> StdResult<T, ()>;
}

impl<T> ResultExt<T> for Result<T> {
    fn show_error(self) -> StdResult<T, ()> {
        self.map_err(|error| error.show_error())
    }
}
