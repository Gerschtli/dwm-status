#![deny(
    missing_debug_implementations, missing_copy_implementations, trivial_casts,
    trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
    unused_qualifications
)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![cfg_attr(feature = "dev", deny(warnings))]

extern crate chrono;
extern crate dbus;
extern crate inotify;
extern crate libnotify;
extern crate uuid;

mod async;
mod error;
#[macro_use]
mod feature;
mod features;
mod io;

use error::*;
use std::collections::HashMap;
use std::env;
use std::sync::mpsc;

fn get_config() -> Result<String> {
    let mut args = env::args();

    let path = args.nth(1)
        .wrap_error("usage", "first parameter config file")?;

    io::read_file(&path).wrap_error("config file", &format!("{} not readable", path))
}

fn render(
    rx: &mpsc::Receiver<async::Message>,
    order: &[String],
    feature_map: &mut HashMap<String, Box<feature::Feature>>,
) -> Result<()> {
    io::render_features(order, feature_map);

    for message in rx {
        match feature_map.get_mut(&message.id) {
            Some(ref mut feature) => {
                feature.update()?;
                println!("update {}: {}", feature.name(), feature.render());
            },
            None => {
                return Err(Error::new_custom(
                    "invalid message",
                    &format!("message id {} does not exist", message.id),
                ))
            },
        };

        io::render_features(order, feature_map);
    }

    Ok(())
}

pub fn run() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let mut features = Vec::new();
    for line in get_config()?.lines() {
        let mut feature = features::create_feature(line, &tx)?;
        feature.init_notifier()?;
        feature.update()?;
        features.push(feature);
    }

    if features.is_empty() {
        return Err(Error::new_custom("empty config", "no features enabled"));
    }

    let order: Vec<_> = features.iter().map(|x| String::from(x.id())).collect();

    let mut feature_map: HashMap<_, _> = features
        .into_iter()
        .map(|feature| (String::from(feature.id()), feature))
        .collect();

    render(&rx, &order, &mut feature_map)
}
