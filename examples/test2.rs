use std::{sync::Arc, thread, time::Duration};
// use std::{thread, time::Duration};

use flow_driver::{CanBus, MotorMode, Command, Motor};

fn main() {
    let canbus1 = Arc::new(CanBus::canbus_init(1, 1, "can1".to_string()));
    let motor1 = Motor::new(1, canbus1.clone());

    motor1.build_and_send_message(Command::BuildAskstatus);

    motor1.build_and_send_message(Command::BuildSpeed(2f32));

    motor1.build_and_send_message(Command::BuildState(3.0));

    motor1.build_and_send_message(Command::BuildSwitchModels(MotorMode::Openloop));

    thread::sleep(Duration::from_secs(1));

    motor1.rdp_msg();

}
