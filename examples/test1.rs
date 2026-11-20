use flow_driver::{self, MotorMode::Openloop};
use std::{thread, time::Duration};
use flow_driver::Command;
use std::sync::{Arc};

fn main(){
    let canbus1 = Arc::new(flow_driver::CanBus::canbus_init(1, 2, "can1".to_string()));

    let motor1 = flow_driver::Motor::new(1, canbus1.clone());
    let motor2 = flow_driver::Motor::new(2, canbus1.clone());
    // 配置总线 将电机挂在在总线上

    motor1.build_and_send_message(Command::BuildAskstatus);
    motor2.build_and_send_message(Command::BuildAskstatus);
    // 询问当前模式

    thread::sleep(Duration::from_secs(1));
    //主线程堵塞1s

    motor1.rdp_msg();
    motor2.rdp_msg();
    // 读取消息并打印

    motor1.build_and_send_message(Command::BuildSwitchModels(Openloop));
    motor2.build_and_send_message(Command::BuildSwitchModels(Openloop));
    // 切换为开环模式

    motor1.build_and_send_message(Command::BuildAskstatus);
    motor2.build_and_send_message(Command::BuildAskstatus);
    // 询问当前模式

    thread::sleep(Duration::from_secs(1));
    // 主线程阻塞1s

    motor1.rdp_msg();
    motor2.rdp_msg();
    // 读取消息并打印

    motor1.build_and_send_message(Command::BuildSpeed(6.0));
    motor2.build_and_send_message(Command::BuildSpeed(6.0));
    // 设置速度为6 rad/s

    motor1.build_and_send_message(Command::BuildOpenloopVoltage(1.0));
    motor2.build_and_send_message(Command::BuildOpenloopVoltage(1.0));
    // 设置电压1 v 

    thread::sleep(Duration::from_secs(2));
    // 主线程阻塞2s

    motor1.build_and_send_message(Command::BuildSpeed(-6.0));
    motor2.build_and_send_message(Command::BuildSpeed(-6.0));
    // 设置速度为-6 rad/s

    thread::sleep(Duration::from_secs(2));
    // 主线程阻塞2s
    motor1.build_and_send_message(Command::BuildOpenloopVoltage(0.0));
    motor2.build_and_send_message(Command::BuildOpenloopVoltage(0.0));
    // 设置电压0 v

    motor1.build_and_send_message(Command::BuildAskstatus);
    motor2.build_and_send_message(Command::BuildAskstatus);
    // 询问当前模式

    thread::sleep(Duration::from_secs(1));
    // 主线程阻塞1s

    motor1.rdp_msg();
    motor2.rdp_msg();
    // 读取消息并打印


    thread::sleep(Duration::from_secs(5));
    // 主线程阻塞5s 在此时间中断会返回错误码

    canbus1.canbus_close();
    println!("close success!");
    // 关闭线程并打印成功

    motor1.rdp_msg();
    motor2.rdp_msg();
    // 读取错误码

}
