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
    let args = env::args();
    let mut features = Vec::new();
    let (tx, rx) = mpsc::channel();

    for arg in args.skip(1) {
        features.push(features::create_feature(&arg, &tx)?);
    }

    if features.is_empty() {
        return Err(Error::new_custom("no args", "no features enabled"));
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
        println!("Message: {:?}", message);

        match feature_map.get_mut(&message.id) {
            Some(ref mut feature) => feature.update()?,
            None => {
                return Err(Error::new_custom(
                    "invalid message",
                    &format!("message id {} does not exist", message.id),
                ))
            }
        }

        io::render_features(&order, &feature_map);
    }

    Ok(())
}
