extern crate chrono;

mod data;
mod system;

fn main() {
    let mut system_info = data::SystemInfo::init();

    system_info.render();
    system_info.listen();
}
