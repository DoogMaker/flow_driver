use std::{sync::Arc, thread, time::Duration};
// use std::{thread, time::Duration};

use flow_driver::{CanBus, MotorMode, Command, Motor};

fn main() {
    // let canbus1 = Arc::new(CanBus::canbus_init(1, 1, "can1".to_string()));
    // let motor1 = Motor::new(1, canbus1.clone());

    // motor1.build_and_send_message(Command::BuildSwitchModels(MotorMode::Position));
    // motor1.build_and_send_message(Command::BuildAskstatus);
    // thread::sleep(Duration::from_secs(1));
    // motor1.rdp_msg();

    // for i in 0..=4 {
    //     let rad: f32 = 0.1 + 0.5 * i as f32;
    //     motor1.build_and_send_message(Command::BuildState(rad));
    //     thread::sleep(Duration::from_secs_f32(1.5));
    //     motor1.build_and_send_message(Command::BuildState(0.0));
    //     thread::sleep(Duration::from_secs_f32(1.5));
    //     println!("this is {i} times running")
    // }

    // motor1.build_and_send_message(Command::BuildAskstatus);
    // thread::sleep(Duration::from_secs(1));
    // motor1.rdp_msg();
    // println!("Position Model is right");

    let canbus1 = Arc::new(CanBus::canbus_init(4, 6, "can0".to_string()));
    let motor7 = Motor::new(4, canbus1.clone());
    let motor8 = Motor::new(5, canbus1.clone());
    let motor9 = Motor::new(6, canbus1.clone());

    motor7.build_and_send_message(Command::BuildSwitchModels(MotorMode::Position));
    motor8.build_and_send_message(Command::BuildSwitchModels(MotorMode::Position));
    motor9.build_and_send_message(Command::BuildSwitchModels(MotorMode::Position));

    motor7.build_and_send_message(Command::BuildAskstatus);
    thread::sleep(Duration::from_millis(20));
    motor7.rdp_msg();

    motor8.build_and_send_message(Command::BuildAskstatus);
    thread::sleep(Duration::from_millis(20));
    motor8.rdp_msg();

    motor9.build_and_send_message(Command::BuildAskstatus);
    thread::sleep(Duration::from_millis(20));
    motor9.rdp_msg();

    motor7.build_and_send_message(Command::BuildState(0.2));
    motor8.build_and_send_message(Command::BuildState(0.2));
    motor9.build_and_send_message(Command::BuildState(0.2));

    thread::sleep(Duration::from_secs(2));

    motor7.rdp_msg();
    motor8.rdp_msg();
    motor9.rdp_msg();

    motor7.build_and_send_message(Command::BuildState(-0.2));
    motor8.build_and_send_message(Command::BuildState(-0.2));
    motor9.build_and_send_message(Command::BuildState(-0.2));

    thread::sleep(Duration::from_secs(2));

    motor7.rdp_msg();
    motor8.rdp_msg();
    motor9.rdp_msg();

    canbus1.canbus_close();
    println!("Close SUCCESS");
}
