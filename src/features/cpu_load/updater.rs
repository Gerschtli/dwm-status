use super::Data;
use super::FEATURE_NAME;
use error::*;
use feature::Updatable;
use io;

const PATH_LOADAVG: &str = "/proc/loadavg";

pub(super) struct Updater;

impl Updatable for Updater {
    type Data = Data;

    fn update(&mut self) -> Result<Self::Data> {
        let content = io::read_file(PATH_LOADAVG)
            .wrap_error(FEATURE_NAME, &format!("failed to read {}", PATH_LOADAVG))?;

        let mut iterator = content.split_whitespace();

        let one = convert_to_float(iterator.next())?;
        let five = convert_to_float(iterator.next())?;
        let fifteen = convert_to_float(iterator.next())?;

        Ok(Data { one, five, fifteen })
    }
}

fn convert_to_float(data: Option<&str>) -> Result<f32> {
    data.wrap_error(FEATURE_NAME, "no data found")?
        .parse()
        .wrap_error(FEATURE_NAME, "could not convert to float")
}
