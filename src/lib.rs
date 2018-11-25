#![deny(
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(feature = "cargo-clippy", deny(warnings))]

extern crate chrono;
extern crate config;
extern crate ctrlc;
extern crate dbus;
extern crate inotify;
extern crate libnotify;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;
extern crate x11;

mod async;
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
        let mut feature = features::create_feature(&feature_name, &tx, &settings)?;
        feature.init_notifier()?;
        features.push(feature);
    }

    if features.is_empty() {
        return Err(Error::new_custom("settings", "no features enabled"));
    }

    resume::init_resume_notifier(&tx)?;

    tx.send(async::Message::UpdateAll)
        .wrap_error("init", "initial update message failed")?;

    ctrlc::set_handler(move || {
        tx.send(async::Message::Kill)
            .wrap_error_kill("termination", "notify thread killed");
    })
    .wrap_error("termination", "failed to set termination handler")?;

    let mut status_bar = StatusBar::new(features)?;

    for message in rx {
        match message {
            async::Message::Kill => break,
            _ => status_bar.update(&message, &settings)?,
        }
    }

    Ok(())
}
