extern crate chrono;
#[macro_use]
extern crate clap;
extern crate libnotify;

mod error;
mod io;
mod features;
mod system;

fn main() {
    let features = io::get_features_from_args();

    println!("{:?}", features);

    let mut feature_objects = Vec::new();
    for feature in &features {
        feature_objects.push(
            match feature {
                &io::FeatureArg::Audio     => features::Feature::Audio(None),
                &io::FeatureArg::Backlight => features::Feature::Backlight(None),
                &io::FeatureArg::Battery   => features::Feature::Battery(None),
                &io::FeatureArg::Time      => features::Feature::Time(None),
            }
        );
    }

    // io::init_notify();
}
