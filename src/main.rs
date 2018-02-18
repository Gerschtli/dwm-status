extern crate chrono;

mod data;

use std::process::Command;

use data::{Init, SystemInfo};

fn main() {
    let system_info = SystemInfo::init();

    println!("{:?}", system_info);
    println!("{}", system_info);

    Command::new("xsetroot").arg("-name").arg(format!("{}", system_info)).output().unwrap();
}
