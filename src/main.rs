#![deny(missing_debug_implementations, missing_copy_implementations, trivial_casts,
        trivial_numeric_casts, unsafe_code, unstable_features, unused_import_braces,
        unused_qualifications)]
#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]
#![cfg_attr(feature = "dev", deny(warnings))]

extern crate chrono;
extern crate libnotify;
extern crate uuid;

mod async;
mod error;
mod feature;
mod features;
mod io;

use error::*;
use std::collections;
use std::env;
use std::ops::DerefMut;
use std::process;
use std::sync::mpsc;

fn main() {
    if let Err(error) = run() {
        error.show_error();
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<_> = env::args().collect();

    if args.len() != 2 {
        return Err(Error::new_custom("wrong usage", "dwm-status <config-file>"));
    }

    let path = &args[1];
    let content =
        io::read_file(path).wrap_error("config file error", &format!("{} not readable", path))?;
    let lines = content.lines();

    let mut features = Vec::new();
    let (tx, rx) = mpsc::channel();

    for line in lines {
        features.push(features::create_feature(line, &tx)?);
    }

    if features.is_empty() {
        return Err(Error::new_custom("empty config", "no features enabled"));
    }

    for mut feature in &mut features {
        feature.update()?;
        feature.init_notifier()?;
    }

    let order: Vec<_> = features.iter().map(|x| String::from(x.id())).collect();

    let mut feature_map: collections::HashMap<String, &mut feature::Feature> =
        collections::HashMap::new();
    for feature in &mut features {
        feature_map.insert(String::from(feature.id()), (*feature).deref_mut());
    }

    io::render_features(&order, &feature_map);

    for message in rx {
        match feature_map.get_mut(&message.id) {
            Some(ref mut feature) => {
                feature.update()?;
                println!("update {}", feature.render());
            }
            None => {
                return Err(Error::new_custom(
                    "invalid message",
                    &format!("message id {} does not exist", message.id),
                ))
            }
        };

        io::render_features(&order, &feature_map);
    }

    Ok(())
}
