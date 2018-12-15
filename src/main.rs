#![deny(
    anonymous_parameters,
    bare_trait_objects,
    elided_lifetimes_in_paths,
    missing_copy_implementations,
    missing_debug_implementations,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    variant_size_differences,
    warnings
)]

extern crate dwm_status;

use std::process;

fn main() {
    if let Err(error) = dwm_status::run() {
        error.show_error();
        process::exit(1);
    }
}
