extern crate chrono;
extern crate libnotify;

mod data;
mod io;
mod system;

fn main() {
    io::init_notify();

    let mut system_info = data::SystemInfo::init();

    system_info.render();
    system_info.listen();
}
