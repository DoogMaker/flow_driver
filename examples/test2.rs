use std::{sync::Arc, thread, time::Duration};
// use std::{thread, time::Duration};

use flow_driver::{CanBus, MotorMode, Command, Motor};

fn main() {
    let canbus1 = Arc::new(CanBus::canbus_init(1, 1, "can1".to_string()));
    let motor1 = Motor::new(1, canbus1.clone());

    motor1.build_and_send_message(Command::BuildSwitchModels(MotorMode::Position));
    motor1.build_and_send_message(Command::BuildAskstatus);
    thread::sleep(Duration::from_secs(1));
    motor1.rdp_msg();

    for i in 0..=4 {
        let rad: f32 = 0.1 + 0.5 * i as f32;
        motor1.build_and_send_message(Command::BuildState(rad));
        thread::sleep(Duration::from_secs_f32(0.5));
        motor1.build_and_send_message(Command::BuildState(0.0));
        thread::sleep(Duration::from_secs_f32(0.5));
        println!("this is {i} times running")
    }

    motor1.build_and_send_message(Command::BuildAskstatus);
    thread::sleep(Duration::from_secs(1));
    motor1.rdp_msg();
    println!("Position Model is right");

}
