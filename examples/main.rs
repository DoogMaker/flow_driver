use flow_driver::Command;
use flow_driver::motor::driver::Motorss;
use flow_driver::{self, MotorMode::Position};
use std::sync::Arc;
use std::{
    thread::{self, Thread},
    time::Duration,
};

fn timesleep(times: u64) {
    thread::sleep(Duration::from_millis(times));
}

struct All_rad {
    lf_rad: [f32; 3],
    rf_rad: [f32; 3],
    lh_rad: [f32; 3],
    rh_rad: [f32; 3],
}
impl All_rad {
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
        target_rad
    }
}

fn main() {
    let motors = Arc::new(Motorss::Motorss_init());

    let allrads = All_rad::init_all_rad();
    let mut target_rad: [f32; 12] = allrads.make_all_rad();

    let mut target_rad: [f32; 12] = [0f32; 12];

    for i in motors.id_s.iter() {
        i.build_and_send_message(Command::BuildSwitchModels(Position));
    }

    timesleep(20);

    for i in motors.id_s.iter() {
        i.build_and_send_message(Command::BuildAskstatus);
    }

    for i in motors.id_s.iter() {
        i.rdp_msg();
    }
    // 切换模式+检查 判断模式是否生效

    for (index, motorr) in motors.id_s.iter().enumerate() {
        if let Err(e) = motorr.move_torad(target_rad[index]) {
            eprintln!("{e}");
        };
    }

    timesleep(20);
}
