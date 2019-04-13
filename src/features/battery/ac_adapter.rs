use super::FEATURE_NAME;
use super::POWER_SUPPLY_PATH;
use crate::error::*;
use crate::wrapper::file;

const AC1: &str = "AC";
const AC2: &str = "ACAD";

pub(super) struct AcAdapter;

impl AcAdapter {
    pub(super) fn get_current() -> Result<&'static str> {
        let ac_name = if Self::ac_exists(AC1) {
            AC1
        } else if Self::ac_exists(AC2) {
            AC2
        } else {
            return Err(Error::new_custom(
                FEATURE_NAME,
                format!(
                    "no ac name ({} or {}) matched in {}",
                    AC1, AC2, POWER_SUPPLY_PATH
                ),
            ));
        };

        Ok(ac_name)
    }

    fn ac_exists(name: &str) -> bool {
        file::exists(&format!("{}/{}", POWER_SUPPLY_PATH, name))
    }
}
