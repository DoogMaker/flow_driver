use flow_driver::{self, Motor, CMD, CanBus, Models};
use std::thread;
use std::sync::Arc;
use std::time::Duration;

fn main() {
    let canbus1: Arc<CanBus> = Arc::new(CanBus::canbus_init(1, 1, "can0".to_string()));

    let motor1: Motor = Motor { id: 1, can_bus: canbus1.clone() };

    motor1.build_send_message(CMD::BuildSwitchModels(Models::Position));
    motor1.build_send_message(CMD::BuildState(0.5f32));
    thread::sleep(Duration::from_millis(500));
    println!("{}", motor1.read_rad());

    motor1.build_send_message(CMD::BuildState(-0.5f32));
    println!("{}", motor1.read_rad());
    thread::sleep(Duration::from_millis(500));
    println!("{}", motor1.read_rad());

    motor1.build_send_message(CMD::BuildState(0f32));
    thread::sleep(Duration::from_millis(500));
    println!("{}", motor1.read_rad());

    thread::sleep(Duration::from_secs(5));
    motor1.rdp_msg();
}
