extern crate chrono;
#[macro_use]
extern crate clap;
extern crate libnotify;

mod data;
mod io;
mod system;

use clap::{Arg, Error};
use data::Feature;
use std::collections::HashMap;
use std::fmt;

arg_enum!{
    #[derive(Copy,Clone,Debug,Eq,Hash,PartialEq)]
    pub enum FeatureArg {
        Audio,
        Backlight,
        Battery,
        Time
    }
}

fn main() {
    let features = get_features();

    println!("{:?}", features);

    let mut feature_objects: Vec<Box<fmt::Display>> = Vec::new();
    for feature in &features {
        feature_objects.push(
            match feature {
                &FeatureArg::Audio     => Box::new(data::audio::Audio::init()),
                &FeatureArg::Backlight => Box::new(data::backlight::Backlight::init()),
                &FeatureArg::Battery   => Box::new(data::battery::Battery::init()),
                &FeatureArg::Time      => Box::new(data::time::Time::init()),
            }
        );
    }

    // io::init_notify();
    //
    // let mut system_info = data::SystemInfo::init();
    //
    // system_info.render();
    // system_info.listen();
}

fn get_features() -> Vec<FeatureArg> {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("features")
                .multiple(true)
                .takes_value(true)
                .required(true)
                .possible_values(&FeatureArg::variants())
                .case_insensitive(true)
                .help("Enabled features in specified order")
        )
        .get_matches();

    let features = values_t!(matches.values_of("features"), FeatureArg)
        .unwrap_or_else(|e| e.exit());

    let uniques: HashMap<_, _> = features.iter()
        .map(|c| (c, ()))
        .collect();

    if features.len() != uniques.len() {
        Error::value_validation_auto(format!("Duplicate features in list: {:?}", features)).exit();
    }

    features.clone()
}
