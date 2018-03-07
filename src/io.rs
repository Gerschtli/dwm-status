use libnotify;
use feature;
use std::collections;
use std::process;

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
