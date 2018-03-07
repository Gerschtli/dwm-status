use feature;
use libnotify;
use std::collections;
use std::fs;
use std::io;
use std::io::Read;
use std::process;
use std::str;

fn read_file(path: &str) -> io::Result<String> {
    let mut s = String::new();
    fs::File::open(path)
        .and_then(|mut f| f.read_to_string(&mut s))
        .map(|_| s)
}

pub fn render_features(order: &[String], feature_map: &collections::HashMap<String, &mut feature::Feature>) {
    let status = order.iter()
        .map(|id| feature_map.get(id).unwrap().render())
        .collect::<Vec<_>>()
        .join(" / ");

    render_status(&status);
}

fn render_status(message: &str) {
    process::Command::new("xsetroot")
        .arg("-name")
        .arg(&message)
        .output()
        .expect("xsetroot failed");
}

pub fn show_notification(summary: &str, body: &str) {
    libnotify::init("dwm-status")
        .expect("init libnotify failed");

    libnotify::Notification::new(summary, Some(body), None)
        .show()
        .expect("show notification failed");

    libnotify::uninit();
}

pub fn value_from_file<T: str::FromStr>(path: &str) -> io::Result<T> {
    try!(read_file(path))
        .trim_right_matches("\n")
        .parse()
        .and_then(|n| Ok(n))
        .or_else(|_| {
            Err(
                io::Error::new(
                    io::ErrorKind::Other,
                   format!("File: \"{}\" doesn't contain an int value", &path)
               )
           )
        })
}
