use flow_driver::{self, Motor, CMD, CanBus, ParamAction, ParamFaultID};
use std::thread;
use std::sync::Arc;
use std::time::Duration;

fn main() {
    let canbus1: Arc<CanBus> = Arc::new(CanBus::canbus_init(1, 6, "can0".to_string()));

    let motors: [Motor; 6] = std::array::from_fn(|i| Motor::new(i as u32 + 1, canbus1.clone()));
    motors[1].canframe_make(CMD::BuildAskstatus);
    thread::sleep(Duration::from_millis(20));
    motors[1].rdp_msg();

    motors[2].build_send_message(CMD::BuildOpenloopVoltage(0.02f32));

    motors[3].build_send_message(
        CMD::BuildGeneralParameters(
            ParamAction::ReadEffectiveval,
            ParamFaultID::None,
            None));

    motors[4].build_send_message(
        CMD::BuildGeneralParameters(
            ParamAction::SaveEffectiveval,
            ParamFaultID::None,
            None));

    motors[5].build_send_message(
        CMD::BuildGeneralParameters(
            ParamAction::WriteTemporaryval,
            ParamFaultID::IPLoopKiLimit,
            Some(2f32)));
}
