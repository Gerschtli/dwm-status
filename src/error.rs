use std::fmt;
pub(crate) use std::result::Result as StdResult;

use log::error;

pub(crate) type Result<T> = StdResult<T, Error>;

#[derive(Debug)]
#[cfg_attr(test, derive(Clone, PartialEq))]
pub struct Error {
    name: String,
    description: String,
    cause: Option<String>,
}

impl Error {
    fn new<N, D, E>(name: N, description: D, cause: E) -> Self
    where
        N: Into<String>,
        D: Into<String>,
        E: fmt::Debug,
    {
        Self {
            name: name.into(),
            description: description.into(),
            cause: Some(format!("{:?}", cause)),
        }
    }

    #[cfg(test)]
    pub(crate) fn new_test<N, D, E>(name: N, description: D, cause: E) -> Self
    where
        N: Into<String>,
        D: Into<String>,
        E: fmt::Debug,
    {
        Self::new(name, description, cause)
    }

    pub(crate) fn new_custom<N, D>(name: N, description: D) -> Self
    where
        N: Into<String>,
        D: Into<String>,
    {
        Self {
            name: name.into(),
            description: description.into(),
            cause: None,
        }
    }

    pub fn show_error(self) {
        error!("{}", self);
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error in {}: {}", self.name, self.description)?;

        if let Some(ref cause) = self.cause {
            write!(f, " ({})", cause)?;
        }

        Ok(())
    }
}

pub(crate) trait WrapErrorExt<T> {
    fn wrap_error<N, D>(self, name: N, description: D) -> Result<T>
    where
        N: Into<String>,
        D: Into<String>;
}

impl<T, E: fmt::Debug> WrapErrorExt<T> for StdResult<T, E> {
    fn wrap_error<N, D>(self, name: N, description: D) -> Result<T>
    where
        N: Into<String>,
        D: Into<String>,
    {
        self.map_err(|error| Error::new(name, description, error))
    }
}

impl<T> WrapErrorExt<T> for Option<T> {
    fn wrap_error<N, D>(self, name: N, description: D) -> Result<T>
    where
        N: Into<String>,
        D: Into<String>,
    {
        self.ok_or_else(|| Error::new_custom(name, description))
    }
}

pub(crate) trait ResultExt<T> {
    fn show_error_and_ignore(self);
}

impl<T> ResultExt<T> for Result<T> {
    fn show_error_and_ignore(self) {
        let _result = self.map_err(Error::show_error);
    }
}

#[cfg(test)]
mod tests {
    use hamcrest2::assert_that;
    use hamcrest2::prelude::*;

    use crate::test_utils::log::Level;
    use crate::test_utils::log::LoggerContext;

    use super::*;

    #[derive(Debug)]
    struct ExampleError;

    mod error {
        use super::*;

        #[test]
        fn new() {
            let logger_context = LoggerContext::new();

            let error = Error::new("name", "description", ExampleError);

            error.show_error();

            logger_context.assert_entry(Level::Error, "Error in name: description (ExampleError)");
        }

        #[test]
        fn new_test() {
            let logger_context = LoggerContext::new();

            let error = Error::new_test("name", "description", ExampleError);

            error.show_error();

            logger_context.assert_entry(Level::Error, "Error in name: description (ExampleError)");
        }

        #[test]
        fn new_custom() {
            let logger_context = LoggerContext::new();

            let error = Error::new_custom("name", "description");

            error.show_error();

            logger_context.assert_entry(Level::Error, "Error in name: description");
        }
    }

    mod wrap_error_ext {
        use super::*;

        mod result {
            use super::*;

            #[test]
            fn when_ok() {
                let result: StdResult<u32, ExampleError> = Ok(42);

                assert_that!(
                    result.wrap_error("name", "description"),
                    is(equal_to(Ok(42)))
                );
            }

            #[test]
            fn when_err() {
                let result: StdResult<u32, ExampleError> = Err(ExampleError);

                assert_that!(
                    result.wrap_error("name", "description"),
                    is(equal_to(Err(Error::new(
                        "name",
                        "description",
                        ExampleError
                    ))))
                );
            }

            #[test]
            fn when_custom_error() {
                let result: StdResult<u32, Error> = Err(Error::new_custom("inner", "inner desc"));

                assert_that!(
                    result.wrap_error("name", "description"),
                    is(equal_to(Err(Error::new(
                        "name",
                        "description",
                        Error::new_custom("inner", "inner desc")
                    ))))
                );
            }
        }

        mod option {
            use super::*;

            #[test]
            fn when_some() {
                let option = Some(42);

                assert_that!(
                    option.wrap_error("name", "description"),
                    is(equal_to(Ok(42)))
                );
            }

            #[test]
            fn when_err() {
                let option: Option<u32> = None;

                assert_that!(
                    option.wrap_error("name", "description"),
                    is(equal_to(Err(Error::new_custom("name", "description"))))
                );
            }
        }
    }

    mod result_ext {
        use super::*;

        #[test]
        fn show_error_when_err() {
            let logger_context = LoggerContext::new();

            let result: Result<u32> = Err(Error::new_custom("name", "description"));

            result.show_error_and_ignore();

            logger_context.assert_entry(Level::Error, "Error in name: description");
        }
    }
}
