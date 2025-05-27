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

use clap::*;
use simplelog::Config;
use simplelog::LevelFilter;
use simplelog::SimpleLogger;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    /// Path to config file
    config_file: String,

    /// Quiet mode (disables INFO logs)
    #[arg(short, long, default_value_t = false)]
    quiet: bool,
}

fn main() {
    let args = Args::parse();

    let log_level = if args.quiet {
        LevelFilter::Warn
    } else {
        LevelFilter::Info
    };

    _ = SimpleLogger::init(log_level, Config::default());

    if let Err(error) = dwm_status::run(&args.config_file) {
        error.show_error();
        process::exit(1);
    }
}
