use libnotify::init;
use libnotify::Notification;
use std::process::Command;

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
