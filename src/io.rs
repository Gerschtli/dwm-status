use feature;
use libnotify;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs;
use std::io;
use std::io::Read;
use std::process;
use std::str;

pub fn read_file(path: &str) -> io::Result<String> {
    let mut s = String::new();
    let mut file = fs::File::open(path)?;
    file.read_to_string(&mut s)?;
    Ok(s)
}

pub fn render_features(
    order: &[String],
    feature_map: &HashMap<String, RefCell<Box<feature::Feature>>>,
) {
    let status = order
        .iter()
        .map(|id| feature_map.get(id).unwrap().borrow().render())
        .collect::<Vec<_>>()
        .join(" / ");

    render_status(&status);
}

pub fn read_int_from_file(path: &str) -> io::Result<i32> {
    read_file(path)?
        .trim_right_matches('\n')
        .parse()
        .or_else(|_| {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("file \"{}\" doesn't contain an int value", &path),
            ))
        })
}

fn render_status(message: &str) {
    process::Command::new("xsetroot")
        .arg("-name")
        .arg(&message)
        .output()
        .expect("xsetroot failed");
}

pub fn show_notification(summary: &str, body: &str, urgency: libnotify::Urgency) {
    libnotify::init("dwm-status").expect("init libnotify failed");

    let notification = libnotify::Notification::new(summary, Some(body), None);
    notification.set_urgency(urgency);
    notification.show().expect("show notification failed");

    libnotify::uninit();
}
