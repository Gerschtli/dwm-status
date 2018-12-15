#![deny(
    anonymous_parameters,
    bare_trait_objects,
    missing_copy_implementations,
    missing_debug_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences
)]
#![cfg_attr(
    feature = "rust-1-31",
    deny(
        clippy::all,
        clippy::complexity,
        clippy::correctness,
        clippy::nursery,
        clippy::pedantic,
        clippy::perf,
        clippy::style,
        elided_lifetimes_in_paths,
        single_use_lifetimes
    )
)]
#![cfg_attr(
    feature = "rust-1-31",
    allow(clippy::filter_map, clippy::non_ascii_literal, deprecated)
)]

extern crate dwm_status;

use std::process;

fn main() {
    if let Err(error) = dwm_status::run() {
        error.show_error();
        process::exit(1);
    }
}
