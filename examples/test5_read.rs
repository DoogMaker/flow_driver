use std::{sync::Arc, thread, time::Duration};
// use std::{thread, time::Duration};

use flow_driver::{CanBus, CMD, Motor, ParamAction, ParamFaulfID};

fn main() {
    let canbus1 = Arc::new(CanBus::canbus_init(2, 2, "can1".to_string()));
    let motor1 = Motor::new(2, canbus1.clone());

    //   EncoderOffset = 0x0308,
    //   EncoderDirection = 0x0309,
    motor1.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::WriteTemporaryval, ParamFaulfID::PoseloopKp, Some(100f32) ));
    motor1.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SubmitEffectiveval, flow_driver::ParamFaulfID::None, None));
    motor1.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::ReadEffectiveval, ParamFaulfID::PoseloopKp, None));
    motor1.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SaveEffectiveval, flow_driver::ParamFaulfID::None, None));

    // thread::sleep(Duration::from_secs(1));
    // motor1.rdp_msg();
    // println!("0308 is right");

    // motor1.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::ReadEffectiveval, Some(ParamFaulfID::EncoderDirection), None ));
    // thread::sleep(Duration::from_secs(1));
    // motor1.rdp_msg();
    // println!("0309 is right");

    thread::sleep(Duration::from_secs(1));

    motor1.rdp_msg();
    motor1.rdp_msg();
    motor1.rdp_msg();
    motor1.rdp_msg();


}    
