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
#![allow(clippy::filter_map, clippy::non_ascii_literal, deprecated)]

use clap::Arg;
use clap::*;
use dwm_status;
use simplelog::Config;
use simplelog::LevelFilter;
use simplelog::SimpleLogger;
use std::process;

fn main() {
    let _ = SimpleLogger::init(LevelFilter::Info, Config::default());

    let matches = app_from_crate!()
        .arg(
            Arg::with_name("config-file")
                .help("Path to config file")
                .required(true),
        )
        .get_matches();

    let config = matches.value_of("config-file").unwrap();

    if let Err(error) = dwm_status::run(config) {
        error.show_error();
        process::exit(1);
    }
}
