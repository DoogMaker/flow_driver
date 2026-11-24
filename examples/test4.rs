use flow_driver::{self, MotorMode::{Position}};
use std::{thread, time::Duration};
use flow_driver::Command;
use std::sync::{Arc};

fn main(){
    let canbus0 = Arc::new(flow_driver::CanBus::canbus_init(6, 6, "can0".to_string()));

    let motor6 = flow_driver::Motor::new(6, canbus0.clone());
    // 配置总线 将电机挂在在总线上
    let pi = std::f32::consts::PI; // f32

    motor6.build_and_send_message(Command::BuildSwitchModels(Position));

    // motor1.build_and_send_message(Command::BuildTorque(0.5));

    motor6.build_and_send_message(Command::BuildAskstatus);

    thread::sleep(Duration::from_secs(1));
    // 主线程阻塞1s

    motor6.rdp_msg();
    // 读取消息并打印

    motor6.build_and_send_message(Command::BuildState(0.2));

    thread::sleep(Duration::from_secs(1));

    motor6.build_and_send_message(Command::BuildState(-0.2));

    thread::sleep(Duration::from_secs(1));

    // if let Err(error) = motor1.move_torad(pi/4f32) {
    //     eprintln!("One{error}");
    // };
    // thread::sleep(Duration::from_secs(2));
    // if let Err(error) = motor1.move_torad(-pi/4f32) {
    //     eprintln!("Two{error}");
    // };

    thread::sleep(Duration::from_secs(2));
    // 主线程阻塞5s 在此时间中断会返回错误码

    canbus0.canbus_close();
    println!("close success!");
    // 关闭线程并打印成功

    motor6.rdp_msg();
    // 读取错误码

}
