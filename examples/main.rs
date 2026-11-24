use flow_driver::{self, MotorMode::{Position}};
use std::{thread::{self, Thread}, time::Duration};
use flow_driver::Command;
use std::sync::{Arc};
use flow_driver::motor::driver::Motorss;

fn timesleep(times: u64) {
    thread::sleep(Duration::from_millis(times));
}

fn main(){
    let motors = Arc::new(Motorss::Motorss_init());
    
    let mut target_rad: [f32; 12] = [0f32; 12];

    for i in motors.id_s.iter() {
        i.build_and_send_message(Command::BuildSwitchModels(Position));
    }

    timesleep(20);

    for (index, motorr) in motors.id_s.iter().enumerate() {
        motorr.build_and_send_message(Command::BuildState(target_rad[index]));
    }

    timesleep(20);

}