use std::{sync::Arc, thread, time::Duration};
// use std::{thread, time::Duration};

use flow_driver::{CanBus, MotorMode, Command, Motor};

fn main() {
    let canbus1 = Arc::new(CanBus::canbus_init(1, 1, "can1".to_string()));
    let motor1 = Motor::new(1, canbus1.clone());

    motor1.build_and_send_message(Command::BuildSwitchModels(MotorMode::Position));
    motor1.build_and_send_message(Command::BuildAskstatus);
    motor1.build_and_send_message(Command::BuildState(3.0));
    thread::sleep(Duration::from_secs(1));
    motor1.rdp_msg();
    motor1.build_and_send_message(Command::BuildState(-3.0));
    thread::sleep(Duration::from_secs(1));
    motor1.rdp_msg();
    println!("Position Model is right");

    motor1.build_and_send_message(Command::BuildSwitchModels(MotorMode::Speed));
    motor1.build_and_send_message(Command::BuildAskstatus);
    motor1.build_and_send_message(Command::BuildSpeed(6.0));
    thread::sleep(Duration::from_secs(1));
    motor1.rdp_msg();
    motor1.build_and_send_message(Command::BuildSpeed(-6.0));
    thread::sleep(Duration::from_secs(1));
    motor1.rdp_msg();
    println!("Speed Models is right");

    thread::sleep(Duration::from_secs(1));

    motor1.rdp_msg();

}
