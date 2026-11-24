use flow_driver::{self, MotorMode::Openloop};
use std::{thread, time::Duration};
use flow_driver::Command;
use std::sync::{Arc};
use flow_driver::motor::driver::Motorss;

fn main() {
    let motors = Arc::new(Motorss::Motorss_init());

    let refresh_handle = {
        let motors = Arc::clone(&motors);
        thread::spawn(move || {
            motors.flash_status();
        })
    };

    thread::sleep(Duration::from_secs(5));

    motors.stop();

    refresh_handle.join().unwrap();

    let rads = motors.now_r.lock().unwrap();
    println!("{:?}", *rads);
}
