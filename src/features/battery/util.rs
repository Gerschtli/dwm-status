use super::FEATURE_NAME;
use super::POWER_SUPPLY_PATH;
use crate::error::*;
use crate::wrapper::file;
use std::time;

pub(super) fn fmt_capacity(capacity: u64) -> String {
    format!("{}%", capacity)
}

pub(super) fn fmt_time(duration: &time::Duration) -> String {
    let minutes = duration.as_secs() / 60;
    format!("{:02}:{:02}", minutes / 60, minutes % 60)
}

#[cfg_attr(all(test, feature = "mocking"), mocktopus::macros::mockable)]
pub(super) fn get_value(device: &str, name: &str) -> Result<u64> {
    file::parse_file_content(format!("{}/{}/{}", POWER_SUPPLY_PATH, device, name))
        .wrap_error(FEATURE_NAME, format!("error reading {}/{}", device, name))
}

pub(super) fn get_value2(device: &str, name1: &str, name2: &str) -> Result<u64> {
    if let Ok(result) = get_value(device, name1) {
        return Ok(result);
    }

    if let Ok(result) = get_value(device, name2) {
        return Ok(result);
    }

    Err(Error::new_custom(
        FEATURE_NAME,
        format!("error reading {}/{} or {}/{}", device, name1, device, name2),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use hamcrest2::assert_that;
    use hamcrest2::prelude::*;
    #[cfg(feature = "mocking")]
    use mocktopus::mocking::*;
    #[cfg(feature = "mocking")]
    use std::io;

    #[test]
    fn fmt_capacity_() {
        assert_that!(fmt_capacity(0), is(equal_to("0%")));
        assert_that!(fmt_capacity(42), is(equal_to("42%")));
        assert_that!(fmt_capacity(101), is(equal_to("101%")));
    }

    #[test]
    fn fmt_time_() {
        assert_that!(
            fmt_time(&time::Duration::from_secs(0)),
            is(equal_to("00:00"))
        );
        assert_that!(
            fmt_time(&time::Duration::from_secs(60)),
            is(equal_to("00:01"))
        );
        assert_that!(
            fmt_time(&time::Duration::from_secs((30 * 60 + 7) * 60)),
            is(equal_to("30:07"))
        );
    }

    #[cfg(feature = "mocking")]
    #[test]
    fn get_value_when_ok() {
        file::parse_file_content.mock_safe(
            |path: String| -> MockResult<_, StdResult<u64, io::Error>> {
                assert_that!(path, is(equal_to("/sys/class/power_supply/device/name")));

                MockResult::Return(Ok(1337))
            },
        );

        assert_that!(get_value("device", "name"), has(1337));
    }

    #[cfg(feature = "mocking")]
    #[test]
    fn get_value_when_err() {
        file::parse_file_content.mock_safe(
            |path: String| -> MockResult<_, StdResult<u64, io::Error>> {
                assert_that!(path, is(equal_to("/sys/class/power_supply/device/name")));

                MockResult::Return(Err(io::Error::new(io::ErrorKind::Other, "io error")))
            },
        );

        assert_that!(
            get_value("device", "name"),
            is(equal_to(Err(Error::new_test(
                "battery",
                "error reading device/name",
                io::Error::new(io::ErrorKind::Other, "io error")
            ))))
        );
    }

    #[cfg(feature = "mocking")]
    #[test]
    fn get_value2_when_first_ok() {
        let mut counter = 0;
        get_value.mock_safe(move |device, name| {
            counter += 1;
            assert_that!(device, is(equal_to("device")));

            match counter {
                1 => {
                    assert_that!(name, is(equal_to("name1")));

                    MockResult::Return(Ok(42))
                },
                _ => panic!("get_value called to often: {} times", counter),
            }
        });

        assert_that!(get_value2("device", "name1", "name2"), has(42));
    }

    #[cfg(feature = "mocking")]
    #[test]
    fn get_value2_when_second_ok() {
        let mut counter = 0;
        get_value.mock_safe(move |device, name| {
            counter += 1;
            assert_that!(device, is(equal_to("device")));

            match counter {
                1 => {
                    assert_that!(name, is(equal_to("name1")));

                    MockResult::Return(Err(Error::new_custom("bla", "io error")))
                },
                2 => {
                    assert_that!(name, is(equal_to("name2")));

                    MockResult::Return(Ok(23))
                },
                _ => panic!("get_value called to often: {} times", counter),
            }
        });

        assert_that!(get_value2("device", "name1", "name2"), has(23));
    }

    #[cfg(feature = "mocking")]
    #[test]
    fn get_value2_when_first_and_second_fail() {
        let mut counter = 0;
        get_value.mock_safe(move |device, name| {
            counter += 1;
            assert_that!(device, is(equal_to("device")));

            match counter {
                1 => {
                    assert_that!(name, is(equal_to("name1")));

                    MockResult::Return(Err(Error::new_custom("bla", "io error")))
                },
                2 => {
                    assert_that!(name, is(equal_to("name2")));

                    MockResult::Return(Err(Error::new_custom("bla2", "io error2")))
                },
                _ => panic!("get_value called to often: {} times", counter),
            }
        });

        assert_that!(
            get_value2("device", "name1", "name2"),
            is(equal_to(Err(Error::new_custom(
                "battery",
                "error reading device/name1 or device/name2"
            ))))
        );
    }
}
