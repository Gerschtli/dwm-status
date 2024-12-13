use std::fmt;
use std::vec::Vec;

use log::debug;
use log::info;
use log::warn;
use serde_derive::Deserialize;

use crate::error::Result;
use crate::error::WrapErrorExt;
use crate::feature;
use crate::wrapper::process;

use super::Data;
use super::FEATURE_NAME;
use super::UpdateConfig;

enum IpAddress {
    V4,
    V6,
}

#[derive(Deserialize, Debug)]
struct Route {
    prefsrc: Option<String>,
}

impl fmt::Display for IpAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IPv{}", match self {
            IpAddress::V4 => 4,
            IpAddress::V6 => 6,
        })
    }
}

pub(super) struct Updater {
    data: Data,
    config: UpdateConfig,
}

impl Updater {
    pub(super) const fn new(data: Data, config: UpdateConfig) -> Self {
        Self { data, config }
    }

    fn get_if_enabled<F: Fn() -> Option<String>>(
        &self,
        enabled: bool,
        builder: F,
    ) -> Option<String> {
        if enabled { builder() } else { None }
    }
}

impl feature::Updatable for Updater {
    fn renderable(&self) -> &dyn feature::Renderable {
        &self.data
    }

    fn update(&mut self) -> Result<()> {
        let ipv4 = self.get_if_enabled(self.config.show_ipv4, || ip_address(&IpAddress::V4));
        let ipv6 = self.get_if_enabled(self.config.show_ipv6, || ip_address(&IpAddress::V6));
        let local_ipv4 = self.get_if_enabled(self.config.show_local_ipv4, || {
            local_address(&IpAddress::V4)
        });
        let local_ipv6 = self.get_if_enabled(self.config.show_local_ipv6, || {
            local_address(&IpAddress::V6)
        });
        let essid = self.get_if_enabled(self.config.show_essid, essid);

        self.data.update(ipv4, ipv6, local_ipv4, local_ipv6, essid);

        Ok(())
    }
}

fn essid() -> Option<String> {
    let command = process::Command::new("iwgetid", &["-r"]);
    let output = command
        .output()
        .wrap_error(FEATURE_NAME, "the essid could not be fetched");

    normalize_output(output)
}

fn local_address(address_type: &IpAddress) -> Option<String> {
    let command = process::Command::new("ip", &[
        match address_type {
            IpAddress::V4 => "-4",
            IpAddress::V6 => "-6",
        },
        "-j",
        "route",
        "show",
        "default",
    ]);

    let output = normalize_output(command.output().wrap_error(
        FEATURE_NAME,
        format!("local ip address {} could not be fetched", address_type),
    ))?;

    let Ok(default_routes) = serde_json::from_str::<Vec<Route>>(&output).wrap_error(
        FEATURE_NAME,
        format!(
            "iproute2 returned badly structured json while we were querying the default {} route.",
            address_type
        ),
    ) else {
        return None;
    };

    let Some(route) = default_routes.first() else {
        warn!("found no default routes");
        return None;
    };

    debug!("choosen default route: {:?}", route);

    route.prefsrc.as_ref().map_or_else(
        || {
            warn!("found no prefered source address for protocol family {address_type}");
            None
        },
        |address| Some(address.to_owned()),
    )
}

fn ip_address(address_type: &IpAddress) -> Option<String> {
    let mut command = process::Command::new("dig", &[
        // decrease time and tries because commands are executed synchronously
        // TODO: make asychronous
        "+time=3",  // default: 5 seconds
        "+tries=1", // default: 3
        "@resolver1.opendns.com",
        "myip.opendns.com",
        "+short",
    ]);

    command.args(match address_type {
        IpAddress::V4 => ["A", "-4"],
        IpAddress::V6 => ["AAAA", "-6"],
    });

    let output = command.output().wrap_error(
        FEATURE_NAME,
        format!("ip address {} could not be fetched", address_type),
    );

    normalize_output(output)
}

fn normalize_output(output: Result<String>) -> Option<String> {
    match output {
        Ok(string) => {
            if string.is_empty() {
                None
            } else {
                Some(string)
            }
        },
        Err(error) => {
            info!("{}", error);
            None
        },
    }
}
