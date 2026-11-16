use flow_driver;
use std::{thread, time::Duration};
use flow_driver::Command;
use flow_driver::{ParamOpcode};
use std::sync::{Arc};

fn main(){
    let canbus1 = Arc::new(flow_driver::CanBus::canbus_init(1, 2, "can1".to_string()));

    let motor1 = flow_driver::Motor::new(1, canbus1.clone());

    motor1.build_and_send_message(Command::BuildSetNodenum(2u8));

    motor1.build_and_send_message(Command::BuildGeneralParameters(ParamOpcode::SubmitEffectiveval, None, None ));

    motor1.build_and_send_message(Command::BuildGeneralParameters(ParamOpcode::SaveEffectiveval, None, None ));

    thread::sleep(Duration::from_secs(1));

    canbus1.canbus_close();
    println!("close success!");
}
