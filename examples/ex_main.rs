use flow_driver::{CMD, ParamAction};
use flow_driver::motor::driver::Motorss;
use flow_driver::{self, Models, ParamFaulfID};
use std::range;

use std::sync::Arc;
use std::{
    thread::{self},
    time::Duration,
};


const LAY: [f32; 12] = [
    0.0, 0.0, 0.0,
    0.0, 0.0, 0.0,
    0.0, 0.0, 0.0,
    0.0, 0.0, 0.0,
    ];

const STAND: [f32; 12] = [
    0.0, 0.0, 0.0,
    0.0, 0.0, 0.0,
    0.0, 0.0, 0.0,
    0.0, 0.0, 0.0,
];

const SIT: [f32; 12] = [
    0.0, 0.0, 0.0,
    0.0, 0.0, 0.0,
    0.0, 0.0, 0.0,
    0.0, 0.0, 0.0,
];

const UPLEG: [f32; 3] = [
    0.0, 0.0, 0.0,
];

const ADVANCELEG: [f32; 3] = [
    0.0, 0.0, 0.0,
];

const BACKLEG: [f32; 3] = [
    0.0, 0.0, 0.0,
];

const SIGN: [f32; 12] = [
    1.0, 1.0, 1.0,
    -1.0, -1.0, -1.0,
    1.0, 1.0, 1.0,
    -1.0, -1.0, -1.0,
];

fn timesleep(times: u64) {
    thread::sleep(Duration::from_millis(times));
}

struct Allrad {
    lf_rad: [f32; 3],
    rf_rad: [f32; 3],
    lh_rad: [f32; 3],
    rh_rad: [f32; 3],
}
impl Allrad {
    fn init_all_rad() -> Self {
        let mut lf_rad: [f32; 3] = [0f32; 3];
        let mut rf_rad: [f32; 3] = [0f32; 3];
        let mut lh_rad: [f32; 3] = [0f32; 3];
        let mut rh_rad: [f32; 3] = [0f32; 3];
        Self {
            lf_rad,
            rf_rad,
            lh_rad,
            rh_rad,
        }
    }

    fn make_all_rad(&self) -> [f32; 12] {
        let mut target_rad: [f32; 12] = [0f32; 12];
        target_rad[0..3].copy_from_slice(&self.lf_rad);
        target_rad[3..6].copy_from_slice(&self.rf_rad);
        target_rad[6..9].copy_from_slice(&self.lh_rad);
        target_rad[9..12].copy_from_slice(&self.rh_rad);
        let target_rad: [f32; 12] = std::array::from_fn(|i| target_rad[i] * SIGN[i]);
        target_rad
    }
}


fn main() {
    let motors = Arc::new(Motorss::Motorss_init());

    let mut allrads = Allrad::init_all_rad();
    let mut target_rad: [f32; 12] = allrads.make_all_rad();

    let mut target_rad: [f32; 12] = STAND;

    for i in motors.id_s.iter() {
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::WriteTemporaryval, ParamFaulfID::PositionErrorThreshold, Some(0.1f32))); // PositionErrorthreshold
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SubmitEffectiveval, ParamFaulfID::None, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::ReadEffectiveval, ParamFaulfID::PoseloopKp, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SaveEffectiveval, ParamFaulfID::None, None));
    }
    thread::sleep(Duration::from_secs(1));

    for i in motors.id_s.iter() {
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::WriteTemporaryval, ParamFaulfID::PoseloopKi, Some(5f32)));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SubmitEffectiveval, ParamFaulfID::None, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::ReadEffectiveval, ParamFaulfID::PoseloopKp, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SaveEffectiveval, ParamFaulfID::None, None));
    }
    thread::sleep(Duration::from_secs(1));

    for i in motors.id_s.iter() {
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::WriteTemporaryval, ParamFaulfID::FeedforwardInertia, Some(0.005f32)));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SubmitEffectiveval, ParamFaulfID::None, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::ReadEffectiveval, ParamFaulfID::PoseloopKp, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SaveEffectiveval, ParamFaulfID::None, None));
    }
    thread::sleep(Duration::from_secs(1));

    for i in motors.id_s.iter() {
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::WriteTemporaryval, ParamFaulfID::PositionErrorTimeouttick, Some(2000f32)));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SubmitEffectiveval, ParamFaulfID::None, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::ReadEffectiveval, ParamFaulfID::PoseloopKp, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SaveEffectiveval, ParamFaulfID::None, None));
    }
    thread::sleep(Duration::from_secs(1));

    for i in motors.id_s.iter() {
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::WriteTemporaryval, ParamFaulfID::SpeedLoopKi, Some(1f32) ));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SubmitEffectiveval, ParamFaulfID::None, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::ReadEffectiveval, ParamFaulfID::PoseloopKp, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SaveEffectiveval, ParamFaulfID::None, None));
    }
    thread::sleep(Duration::from_secs(1));

    for i in motors.id_s.iter() {
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::WriteTemporaryval, ParamFaulfID::PoseloopKp, Some(100f32) ));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SubmitEffectiveval, ParamFaulfID::None, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::ReadEffectiveval, ParamFaulfID::PoseloopKp, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SaveEffectiveval, ParamFaulfID::None, None));
    }
    thread::sleep(Duration::from_secs(1));

    for i in motors.id_s.iter() {
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::WriteTemporaryval, ParamFaulfID::PositionErrorThreshold, Some(0.4f32) ));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SubmitEffectiveval, ParamFaulfID::None, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::ReadEffectiveval, ParamFaulfID::PositionErrorThreshold, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SaveEffectiveval, ParamFaulfID::None, None));
    }
    thread::sleep(Duration::from_secs(1));

    for i in motors.id_s.iter() {
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::WriteTemporaryval, ParamFaulfID::SpeedLoopKi, Some(1f32) ));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SubmitEffectiveval, ParamFaulfID::None, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::ReadEffectiveval, ParamFaulfID::SpeedLoopKi, None));
        i.build_and_send_message(CMD::BuildGeneralParameters(ParamAction::SaveEffectiveval, ParamFaulfID::None, None));
    }
    thread::sleep(Duration::from_secs(1));

    for _ in 0..=5 {
        for i in motors.id_s.iter() {
            i.rdp_msg();
        }
    }

    // for i in motors.id_s.iter() {
    //     i.build_and_send_message(Command::BuildSwitchModels(Position));
    // }

    // timesleep(20);

    // for i in motors.id_s.iter() {
    //     i.build_and_send_message(Command::BuildAskstatus);
    // }

    // timesleep(50);

    // for i in motors.id_s.iter() {
    //     i.rdp_msg();
    // }
    // // 切换模式+检查 判断模式是否生效

    // allrads.lf_rad = [0.0, 1.0, 1.0];
    // target_rad = allrads.make_all_rad();
    // for (index, motorr) in motors.id_s.iter().enumerate() {
    //     if let Err(e) = motorr.move_torad(target_rad[index]) {
    //         eprintln!("{e}");
    //     };
    // }
    // timesleep(50);
    // println!("lf finish!");

    // allrads.rf_rad = [0.0, 1.0, 1.0];
    // target_rad = allrads.make_all_rad();
    // for (index, motorr) in motors.id_s.iter().enumerate() {
    //     if let Err(e) = motorr.move_torad(target_rad[index]) {
    //         eprintln!("{e}");
    //     };
    // }
    // timesleep(50);
    // println!("rf finish!");

    // allrads.lh_rad = [0.0, 1.0, 1.0];
    // target_rad = allrads.make_all_rad();
    // for (index, motorr) in motors.id_s.iter().enumerate() {
    //     if let Err(e) = motorr.move_torad(target_rad[index]) {
    //         eprintln!("{e}");
    //     };
    // }
    // timesleep(50);
    // println!("lh finish!");


    // allrads.rh_rad = [0.0, 1.0, 1.0];
    // target_rad = allrads.make_all_rad();
    // for (index, motorr) in motors.id_s.iter().enumerate() {
    //     if let Err(e) = motorr.move_torad(target_rad[index]) {
    //         eprintln!("{e}");
    //     };
    // }
    // timesleep(50);
    // println!("rh finish!");

    motors.stop();
    println!("close success!")

}
