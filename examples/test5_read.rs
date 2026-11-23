use std::{sync::Arc, thread, time::Duration};
// use std::{thread, time::Duration};

use flow_driver::{CanBus, MotorMode, Command, Motor, ParamOpcode, ParameterResponseFrame, ParamId};

fn main() {
    let canbus1 = Arc::new(CanBus::canbus_init(2, 2, "can1".to_string()));
    let motor1 = Motor::new(2, canbus1.clone());

    //   EncoderOffset = 0x0308,
    //   EncoderDirection = 0x0309,
    motor1.build_and_send_message(Command::BuildGeneralParameters(ParamOpcode::WriteTemporaryval, Some(ParamId::PoseloopKp), Some(100f32) ));
    motor1.build_and_send_message(Command::BuildGeneralParameters(ParamOpcode::SubmitEffectiveval, None, None));
    motor1.build_and_send_message(Command::BuildGeneralParameters(ParamOpcode::ReadEffectiveval, Some(ParamId::PoseloopKp), None));
    motor1.build_and_send_message(Command::BuildGeneralParameters(ParamOpcode::SaveEffectiveval, None, None));

    // thread::sleep(Duration::from_secs(1));
    // motor1.rdp_msg();
    // println!("0308 is right");

    // motor1.build_and_send_message(Command::BuildGeneralParameters(ParamOpcode::ReadEffectiveval, Some(ParamId::EncoderDirection), None ));
    // thread::sleep(Duration::from_secs(1));
    // motor1.rdp_msg();
    // println!("0309 is right");

    thread::sleep(Duration::from_secs(1));

    motor1.rdp_msg();
    motor1.rdp_msg();
    motor1.rdp_msg();
    motor1.rdp_msg();


}    
