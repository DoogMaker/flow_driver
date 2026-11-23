use std::{sync::Arc, thread, time::Duration};
// use std::{thread, time::Duration};

use flow_driver::{CanBus, MotorMode, Command, Motor, ParamOpcode, ParameterResponseFrame, ParamId};

fn main() {
    let canbus1 = Arc::new(CanBus::canbus_init(1, 1, "can1".to_string()));
    let motor1 = Motor::new(1, canbus1.clone());

    //   EncoderOffset = 0x0308,
    //   EncoderDirection = 0x0309,
    motor1.build_and_send_message(Command::BuildGeneralParameters(ParamOpcode::ReadEffectiveval, Some(ParamId::EncoderOffset), None ));
    thread::sleep(Duration::from_secs(1));
    motor1.rdp_msg();
    println!("0308 is right");

    motor1.build_and_send_message(Command::BuildGeneralParameters(ParamOpcode::ReadEffectiveval, Some(ParamId::EncoderDirection), None ));
    thread::sleep(Duration::from_secs(1));
    motor1.rdp_msg();
    println!("0309 is right");

    thread::sleep(Duration::from_secs(1));

    motor1.rdp_msg();

}    
