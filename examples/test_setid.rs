use flow_driver;
use std::{thread, time::Duration};
use flow_driver::Command;
use flow_driver::{ParamOpcode};
use std::sync::{Arc};
use std::env;

fn get_target_id() -> u32 {
    let arg = env::args().nth(1).unwrap_or_default();
    let arg = arg.parse::<u32>().unwrap_or_default();
    arg
}

fn main(){
    let target_id:u32 = get_target_id();
    let canbus1 = Arc::new(flow_driver::CanBus::canbus_init(1, target_id , "can1".to_string()));

    let motor1 = flow_driver::Motor::new(1, canbus1.clone());
    let motor2 = flow_driver::Motor::new(target_id, canbus1.clone());

    motor1.build_and_send_message(Command::BuildSetNodenum(target_id as u8));

    motor2.build_and_send_message(Command::BuildGeneralParameters(ParamOpcode::SubmitEffectiveval, None, None ));

    motor2.build_and_send_message(Command::BuildGeneralParameters(ParamOpcode::SaveEffectiveval, None, None ));

    thread::sleep(Duration::from_secs(1));

    motor2.build_and_send_message(Command::BuildAskstatus);
    thread::sleep(Duration::from_secs(1));
    motor2.rdp_msg();
    thread::sleep(Duration::from_secs(1));

    canbus1.canbus_close();
    println!("close success!");
}
