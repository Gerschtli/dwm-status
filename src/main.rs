extern crate chrono;

mod data;

fn main() {
    let mut system_info = data::SystemInfo::init();

    system_info.render();
    system_info.listen();
}
