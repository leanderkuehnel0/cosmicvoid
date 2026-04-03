use dbus::blocking::Connection;
use get_sysinfo::Info;

use std::{thread::sleep, time::Duration};

fn main() {
    let dbus_connection = Connection::new_system().unwrap();
    let mut info = Info::new(&dbus_connection);
    loop {
        info.update();
        sleep(Duration::from_millis(1000));
    }
}
