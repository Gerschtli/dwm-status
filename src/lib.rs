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
    variant_size_differences,
    warnings
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

extern crate chrono;
extern crate config;
extern crate ctrlc;
extern crate dbus;
extern crate inotify;
extern crate libnotify;
#[macro_use]
extern crate serde_derive;
extern crate uuid;
extern crate x11;

mod communication;
mod error;
#[macro_use]
mod feature;
mod features;
mod io;
mod resume;
mod settings;
mod status_bar;
mod utils;
mod wrapper;

use error::*;
use status_bar::StatusBar;
use std::env;
use std::sync::mpsc;

fn get_settings() -> Result<settings::Settings> {
    let mut args = env::args();

    let path = args
        .nth(1)
        .wrap_error("usage", "first parameter config file")?;

    settings::Settings::new(&path).wrap_error("settings", "error creating settings object")
}

pub fn run() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let settings = get_settings()?;
    let mut features = Vec::new();

    for feature_name in &settings.order {
        let mut feature = features::create_feature(feature_name, &tx, &settings)?;
        feature.init_notifier()?;
        features.push(feature);
    }

    if features.is_empty() {
        return Err(Error::new_custom("settings", "no features enabled"));
    }

    resume::init_resume_notifier(&tx)?;

    tx.send(communication::Message::UpdateAll)
        .wrap_error("init", "initial update message failed")?;

    ctrlc::set_handler(move || {
        tx.send(communication::Message::Kill)
            .wrap_error_kill("termination", "notify thread killed");
    })
    .wrap_error("termination", "failed to set termination handler")?;

    let mut status_bar = StatusBar::new(features)?;

    for message in rx {
        match message {
            communication::Message::Kill => break,
            _ => status_bar.update(&message, &settings)?,
        }
    }

    Ok(())
}
