use flow_driver;
use std::{thread, time::Duration};
use flow_driver::Command;
use std::sync::{Arc};

fn main(){
    let canbus1 = Arc::new(flow_driver::CanBus::canbus_init(1, 2, "can0".to_string()));

    let motor1 = flow_driver::Motor::new(1, canbus1.clone());
    let motor2 = flow_driver::Motor::new(2, canbus1.clone());

    let msg = motor1.build_send_message(Command::build_askstatus);
    motor1.send_messages(msg);

    let msg = motor2.build_send_message(Command::build_askstatus);
    motor2.send_messages(msg);

    thread::sleep(Duration::from_secs(5));

    canbus1.canbus_close();
    println!("clse success!");
}
