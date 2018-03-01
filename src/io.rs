use clap::{Arg, Error};
use libnotify::{init, Notification};
use std::collections::HashMap;
use std::process::Command;

arg_enum!{
    #[derive(Copy,Clone,Debug,Eq,Hash,PartialEq)]
    pub enum FeatureArg {
        Audio,
        Backlight,
        Battery,
        Time
    }
}

pub fn get_features_from_args() -> Vec<FeatureArg> {
    let matches = app_from_crate!()
        .arg(
            Arg::with_name("features")
                .multiple(true)
                .takes_value(true)
                .required(true)
                .possible_values(&FeatureArg::variants())
                .case_insensitive(true)
                .help("Unique list of enabled features in specified order")
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

pub fn init_notify() {
    init("dwm-status").expect("init libnotify failed");
}

pub fn render_status(message: &str) {
    Command::new("xsetroot")
        .arg("-name")
        .arg(&message)
        .output()
        .expect("xsetroot failed");
}

pub fn show_notification(summary: &str, body: &str) {
    Notification::new(summary, Some(body), None)
        .show()
        .expect("show notification failed");
}
