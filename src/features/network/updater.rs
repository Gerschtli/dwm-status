use super::Data;
use super::UpdateConfig;
use error::*;
use feature;
use wrapper::process;

enum IpAddress {
    V4,
    V6,
}

pub(super) struct Updater {
    data: Data,
    config: UpdateConfig,
}

impl Updater {
    pub(super) fn new(data: Data, config: UpdateConfig) -> Self {
        Self { data, config }
    }

    fn get_if_enabled<F: Fn() -> Result<Option<String>>>(
        &self,
        enabled: bool,
        builder: F,
    ) -> Result<Option<String>> {
        if enabled {
            builder()
        } else {
            Ok(None)
        }
    }
}

impl feature::Updatable for Updater {
    fn renderable(&self) -> Box<&dyn feature::Renderable> {
        Box::new(&self.data)
    }

    fn update(&mut self) -> Result<()> {
        let ipv4 = self.get_if_enabled(self.config.show_ipv4, || ip_address(IpAddress::V4))?;
        let ipv6 = self.get_if_enabled(self.config.show_ipv6, || ip_address(IpAddress::V6))?;
        let essid = self.get_if_enabled(self.config.show_essid, essid)?;

        self.data.update(ipv4, ipv6, essid);

        Ok(())
    }
}

fn essid() -> Result<Option<String>> {
    let command = process::Command::new("iwgetid", &["-r"]);
    let output = command.output()?;

    Ok(normalize_output(output))
}

fn ip_address(address_type: IpAddress) -> Result<Option<String>> {
    let command = process::Command::new(
        "dig",
        &[
            "@resolver1.opendns.com",
            "A",
            "myip.opendns.com",
            "+short",
            match address_type {
                IpAddress::V4 => "-4",
                IpAddress::V6 => "-6",
            },
        ],
    );

    let output = command.output()?;

    Ok(normalize_output(output))
}

fn normalize_output(output: String) -> Option<String> {
    if output.is_empty() {
        None
    } else {
        Some(output)
    }
}
