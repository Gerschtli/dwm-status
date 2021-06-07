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
    clippy::use_self
)]
#![cfg_attr(all(test, feature = "mocking"), allow(trivial_casts, unsafe_code))]
#![cfg_attr(all(test, feature = "mocking"), feature(proc_macro_hygiene))]

use std::collections::HashSet;

use crate::error::Error;
use crate::error::Result;
use crate::error::ResultExt;
use crate::status_bar::StatusBar;
use crate::wrapper::channel;
use crate::wrapper::termination;

#[macro_use]
mod macros;
mod communication;
mod error;
mod feature;
mod features;
mod resume;
mod settings;
mod status_bar;
#[cfg(test)]
mod test_utils;
mod utils;
mod wrapper;

fn validate_settings(settings: &settings::Settings) -> Result<()> {
    if settings.general.order.is_empty() {
        return Err(Error::new_custom("settings", "no features enabled"));
    }

    let set: HashSet<&String> = settings.general.order.iter().collect();
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
            .show_error_and_ignore();
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
