use flow_driver::{self, MotorMode::{IDLE, Position}};
use std::{thread, time::Duration};
use flow_driver::Command;
use std::sync::{Arc};

fn main(){
    let canbus0 = Arc::new(flow_driver::CanBus::canbus_init(8, 8, "can1".to_string()));

    let motor6 = flow_driver::Motor::new(8, canbus0.clone());
    // 配置总线 将电机挂在在总线上
    let pi = std::f32::consts::PI; // f32

    motor6.build_and_send_message(Command::BuildSwitchModels(Position));
    motor6.build_and_send_message(Command::BuildAskstatus);
    thread::sleep(Duration::from_secs(1));
    motor6.rdp_msg();

    motor6.build_and_send_message(Command::BuildState(0.4));
    thread::sleep(Duration::from_secs(1));

    motor6.build_and_send_message(Command::BuildState(-0.4));
    thread::sleep(Duration::from_secs(1));

    motor6.build_and_send_message(Command::BuildState(0.4));
    thread::sleep(Duration::from_secs(1));

    motor6.build_and_send_message(Command::BuildState(-0.4));
    thread::sleep(Duration::from_secs(1));

    motor6.build_and_send_message(Command::BuildState(0.4));
    thread::sleep(Duration::from_secs(1));

    motor6.build_and_send_message(Command::BuildState(-0.4));
    thread::sleep(Duration::from_secs(1));

    motor6.build_and_send_message(Command::BuildSwitchModels(IDLE));
    motor6.build_and_send_message(Command::BuildAskstatus);
    thread::sleep(Duration::from_secs(1));
    motor6.rdp_msg();

    canbus0.canbus_close();
    println!("close success!");

    motor6.rdp_msg();

}
