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
#![cfg_attr(all(test, feature = "mocking"), allow(trivial_casts))]
#![cfg_attr(
    all(test, feature = "mocking"),
    feature(custom_attribute, proc_macro_hygiene)
)]

extern crate chrono;
extern crate config;
extern crate ctrlc;
extern crate dbus;
#[cfg(test)]
#[macro_use]
extern crate hamcrest2;
extern crate inotify;
extern crate libnotify;
#[macro_use]
extern crate log;
#[cfg(all(test, feature = "mocking"))]
extern crate mocktopus;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate x11;

#[macro_use]
mod macros;
mod communication;
mod error;
mod feature;
mod features;
mod resume;
mod settings;
mod status_bar;
mod utils;
mod wrapper;

use error::*;
use status_bar::StatusBar;
use std::collections::HashSet;
use std::iter::FromIterator;
use wrapper::channel;
use wrapper::termination;

fn validate_settings(settings: &settings::Settings) -> Result<()> {
    if settings.general.order.is_empty() {
        return Err(Error::new_custom("settings", "no features enabled"));
    }

    let set: HashSet<&String> = HashSet::from_iter(settings.general.order.iter());
    if set.len() < settings.general.order.len() {
        return Err(Error::new_custom(
            "settings",
            "order must not have more than one entry of one feature",
        ));
    }

    Ok(())
}

pub fn run(config_path: &str) -> Result<()> {
    let settings = settings::Settings::init(config_path)?;

    validate_settings(&settings)?;

    let (sender, receiver) = channel::create();
    let mut features = Vec::new();

    for (index, feature_name) in settings.general.order.iter().enumerate() {
        let mut feature = features::create_feature(index, feature_name, &sender, &settings)?;
        feature.init_notifier()?;
        features.push(feature);
    }

    resume::init_resume_notifier(&sender)?;

    sender.send(communication::Message::UpdateAll)?;

    termination::register_handler(move || {
        sender
            .send(communication::Message::Kill)
            .show_error()
            .unwrap()
    })?;

    let mut status_bar = StatusBar::init(features)?;

    while let Ok(message) = receiver.read_blocking() {
        match message {
            communication::Message::Kill => break,
            _ => status_bar.update(&message, &settings.general)?,
        }
    }

    Ok(())
}
