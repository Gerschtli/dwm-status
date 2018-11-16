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
extern crate dbus as dbus_lib;
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
mod dbus;
mod features;
mod io;
mod resume;
mod settings;
mod status_bar;
mod utils;

use error::*;
use status_bar::StatusBar;
use std::collections::HashMap;
use std::env;
use std::ops::DerefMut;
use std::sync::mpsc;

fn get_settings() -> Result<settings::Settings> {
    let mut args = env::args();

    let path = args
        .nth(1)
        .wrap_error("usage", "first parameter config file")?;

    settings::Settings::new(&path).wrap_error("settings", "error creating settings object")
}

fn render(
    tx: &mpsc::Sender<async::Message>,
    rx: &mpsc::Receiver<async::Message>,
    order: &[uuid::Uuid],
    feature_map: &mut HashMap<uuid::Uuid, Box<feature::Feature>>,
    settings: &settings::Settings,
) -> Result<()> {
    let tx = tx.clone();
    ctrlc::set_handler(move || {
        tx.send(async::Message::Kill)
            .wrap_error_kill("termination", "notify thread killed");
    })
    .wrap_error("termination", "failed to set termination handler")?;

    let status_bar = StatusBar::new(settings.separator.clone())?;
    status_bar.render(order, feature_map)?;

    for message in rx {
        match message {
            async::Message::FeatureUpdate(ref id) => {
                match feature_map.get_mut(id) {
                    Some(ref mut feature) => {
                        update_feature(feature.deref_mut().deref_mut(), settings.debug)?;
                    },
                    None => {
                        return Err(Error::new_custom(
                            "invalid message",
                            &format!("message id {} does not exist", id),
                        ))
                    },
                };

                status_bar.render(order, feature_map)?;
            },
            async::Message::Kill => break,
            async::Message::UpdateAll => {
                if settings.debug {
                    println!("update all");
                }
                for feature in feature_map.values_mut() {
                    update_feature(feature.deref_mut(), settings.debug)?;
                }
            },
        }
    }

    Ok(())
}

fn update_feature(feature: &mut feature::Feature, debug: bool) -> Result<()> {
    feature.update()?;

    if debug {
        println!("update {}: {}", feature.name(), feature.render());
    }

    Ok(())
}

pub fn run() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let settings = get_settings()?;
    let mut features = Vec::new();

    for feature_name in &settings.order {
        let mut feature = features::create_feature(&feature_name, &tx, &settings)?;
        feature.init_notifier()?;
        feature.update()?;
        features.push(feature);
    }

    if features.is_empty() {
        return Err(Error::new_custom("settings", "no features enabled"));
    }

    let order: Vec<_> = features.iter().map(|x| x.id()).collect();

    let mut feature_map: HashMap<_, _> = features
        .into_iter()
        .map(|feature| (feature.id(), feature))
        .collect();

    resume::init_resume_notifier(&tx)?;

    render(&tx, &rx, &order, &mut feature_map, &settings)
}
