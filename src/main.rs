#![deny(
    anonymous_parameters,
    bare_trait_objects,
    clippy::all,
    clippy::complexity,
    clippy::correctness,
    clippy::nursery,
    clippy::pedantic,
    clippy::perf,
    clippy::style,
    elided_lifetimes_in_paths,
    missing_copy_implementations,
    missing_debug_implementations,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]
#![allow(
    clippy::missing_errors_doc,
    clippy::non_ascii_literal,
    clippy::redundant_pub_crate,
    clippy::unused_self,
    clippy::wildcard_imports
)]

use std::process;

use clap::Arg;
use clap::*;
use simplelog::Config;
use simplelog::LevelFilter;
use simplelog::SimpleLogger;

fn main() {
    let _ = SimpleLogger::init(LevelFilter::Info, Config::default());

    let matches = build_app().get_matches();

    let default_conf_path = format!("{}{}", env!("XDG_CONFIG_HOME"), "/dwm-status/defaults.yml");

    let config = matches
        .value_of("config-file")
        .unwrap_or(&default_conf_path);

    if let Err(error) = dwm_status::run(config) {
        error.show_error();
        process::exit(1);
    }
}

fn build_app() -> Command<'static> {
    command!().arg(
        Arg::new("config-file")
            .help("Path to config file")
            .required(false),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_app() {
        build_app().debug_assert();
    }
}
