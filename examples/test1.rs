<<<<<<< HEAD
use flow_driver::{self, MotorMode::{Openloop, Position}};
=======
use flow_driver::{self, Models::Openloop};
>>>>>>> feature/0.2.0
use std::{thread, time::Duration};
use flow_driver::CMD;
use std::sync::{Arc};

fn main(){
    let canbus1 = Arc::new(flow_driver::CanBus::canbus_init(1, 2, "can1".to_string()));

    let motor1 = flow_driver::Motor::new(1, canbus1.clone());
    // 配置总线 将电机挂在在总线上

    motor1.build_and_send_message(Command::BuildAskstatus);

    thread::sleep(Duration::from_secs(1));
    motor1.rdp_msg();

    motor1.build_and_send_message(Command::BuildSwitchModels(Position));
    motor1.build_and_send_message(Command::BuildState(-0.4));
    thread::sleep(Duration::from_secs(5));

    motor1.build_and_send_message(Command::BuildAskstatus);
    motor1.rdp_msg();

    thread::sleep(Duration::from_secs(1));
    // 主线程阻塞1s 在此时间中断会返回错误码

    canbus1.canbus_close();
    println!("close success!");

    motor1.rdp_msg();

}
