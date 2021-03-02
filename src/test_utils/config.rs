use std::collections::HashMap;

use hamcrest2::assert_that;
use hamcrest2::prelude::*;
use mocktopus::mocking::*;

use crate::error::Error;
use crate::error::Result;
use crate::settings;
use crate::wrapper::config;
use crate::wrapper::config::Value;

pub(crate) fn test_set_default_ok<T: settings::ConfigType>(
    name: &'static str,
    default_map_builder: fn() -> HashMap<String, Value>,
) {
    test_set_default::<T>(name, default_map_builder, Ok(()));
}

pub(crate) fn test_set_default_err<T: settings::ConfigType>(
    name: &'static str,
    default_map_builder: fn() -> HashMap<String, Value>,
) {
    test_set_default::<T>(
        name,
        default_map_builder,
        Err(Error::new_custom("name", "description")),
    );
}

fn test_set_default<T: settings::ConfigType>(
    name: &'static str,
    default_map_builder: fn() -> HashMap<String, Value>,
    result: Result<()>,
) {
    let result_ = result.clone();

    config::Config::set_default.mock_safe(move |_, key, value: HashMap<String, Value>| {
        assert_that!(key, is(equal_to(name)));
        assert_that!(value, is(equal_to(default_map_builder())));

        MockResult::Return(result_.clone())
    });

    let mut config = config::Config::new();

    assert_that!(T::set_default(&mut config), is(equal_to(result)));
}
