use flow_driver::Models;
use flow_driver::motor::driver::Motorss;
use std::sync::Arc;
use std::{
    thread::{self},
    time::Duration,
};
use flow_driver::{CMD, ParamAction, ParamFaulfID};

const TIMEOUT: u64 = 8;
const LAY: [f32; 12] = [
    -0.0, -0.0, -0.0,
    0.0, 0.0, 0.0,
    0.0, -0.0, -0.0,
    -0.0, 0.0, 0.0,
    ];

const STAND1: [f32; 12] = [
    -0.0, 0.9, 0.9,
    0.0, -0.9, -0.9,
    0.0, 0.9, 0.9,
    -0.0, -0.9, -0.9,
];

const STAND2: [f32; 12] = [
    0.0, 0.8, 1.0,
    0.0, -0.8, -1.0,
    0.0, 0.8, 1.0,
    0.0, -0.8, -1.0,
];

const SIT: [f32; 12] = [
    -0.4, 0.4, 0.2,
    0.4, -0.4, -0.2,
    0.4, 0.4, 0.4,
    -0.4, -0.4, -0.4,
];

const UPLEG: [f32; 3] = [
    -0.4, 0.4, -0.4,
];

const ADVANCELEG: [f32; 3] = [
    -0.4, 0.0, 0.2,
];

const BACKLEG: [f32; 3] = [
    0.0, 0.0, 0.0,
];

const SIGN: [f32; 12] = [
    -1.0, 1.0, 1.0,
    1.0, -1.0, -1.0,
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
        let mut lf_rad: [f32; 3] = [0.0f32; 3];
        let mut rf_rad: [f32; 3] = [0.0f32; 3];
        let mut lh_rad: [f32; 3] = [0.0f32; 3];
        let mut rh_rad: [f32; 3] = [0.0f32; 3];
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

    for i in motors.id_s.iter() {
        i.build_and_send_message(CMD::BuildSwitchModels(Models::Position));
        i.build_and_send_message(CMD::BuildAskstatus);
    }
    thread::sleep(Duration::from_secs(1));

    target_rad = STAND1;
    for (index, motorr) in motors.id_s.iter().enumerate() {
        motorr.build_and_send_message(CMD::BuildState(target_rad[index]));
        motorr.build_and_send_message(CMD::BuildAskstatus);
    }
    thread::sleep(Duration::from_secs(TIMEOUT));
//
//     target_rad = STAND2;
//         for (index, motorr) in motors.id_s.iter().enumerate() {
//             motorr.build_and_send_message(CMD::BuildState(target_rad[index]));
//             motorr.build_and_send_message(CMD::BuildAskstatus);
//         }
//         thread::sleep(Duration::from_secs(TIMEOUT));

    target_rad = LAY;
    for (index, motorr) in motors.id_s.iter().enumerate() {
        motorr.build_and_send_message(CMD::BuildState(target_rad[index]));
        motorr.build_and_send_message(CMD::BuildAskstatus);
    }
    thread::sleep(Duration::from_secs(TIMEOUT));

    motors.stop();
    println!("close success!");

    for i in motors.id_s.iter() {
        i.rdp_msg();
    }
}
