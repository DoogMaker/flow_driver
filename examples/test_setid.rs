use flow_driver;
use std::{thread, time::Duration};
use flow_driver::Command;
use flow_driver::{ParamId, ParamOpcode, MotorMode};
use std::sync::{Arc};

fn main(){
    let canbus1 = Arc::new(flow_driver::CanBus::canbus_init(1, 2, "can1".to_string()));

    let motor1 = flow_driver::Motor::new(1, canbus1.clone());

    let msg = motor1.build_send_message(Command::build_set_nodenum(2u8));
    motor1.send_messages(msg);

    let msg = motor1.build_send_message(Command::build_general_parameters(ParamOpcode::SubmitEffectiveval, None, None ));
    motor1.send_messages(msg);

    let msg = motor1.build_send_message(Command::build_general_parameters(ParamOpcode::SaveEffectiveval, None, None ));
    motor1.send_messages(msg);

    thread::sleep(Duration::from_secs(1));

    canbus1.canbus_close();
    println!("close success!");
}
