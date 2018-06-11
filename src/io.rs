use libnotify;
use std::fs;
use std::io;
use std::io::Read;
use std::str;

pub fn read_file(path: &str) -> io::Result<String> {
    let mut s = String::new();
    let mut file = fs::File::open(path)?;
    file.read_to_string(&mut s)?;
    Ok(s)
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

pub fn show_notification(summary: &str, body: &str, urgency: libnotify::Urgency) {
    libnotify::init("dwm-status").expect("init libnotify failed");

    let notification = libnotify::Notification::new(summary, Some(body), None);
    notification.set_urgency(urgency);
    notification.show().expect("show notification failed");

    libnotify::uninit();
}
