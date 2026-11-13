use flow_driver::{self, MotorMode::Openloop};
use std::{thread, time::Duration};
use flow_driver::Command;
use std::sync::{Arc};

fn main(){
    let canbus1 = Arc::new(flow_driver::CanBus::canbus_init(1, 2, "can1".to_string()));

    let motor1 = flow_driver::Motor::new(1, canbus1.clone());
    let motor2 = flow_driver::Motor::new(2, canbus1.clone());

    let msg = motor1.build_send_message(Command::build_askstatus);
    motor1.send_messages(msg);

    let msg = motor2.build_send_message(Command::build_askstatus);
    motor2.send_messages(msg);

    thread::sleep(Duration::from_secs(1));

    let msg1 = match motor1.recv_msg() {
        Some((return_type, msg)) => (return_type, msg),
        None => {
            println!("recv_msg None");
            (0, [0; 8])
        }
    };
    motor1.decode_msg(msg1.0, msg1.1);

    let msg2 = match motor2.recv_msg() {
        Some((return_type, msg)) => (return_type, msg),
        None => {
            println!("recv_msg None");
            (0, [0; 8])
        }
    };
    motor1.decode_msg(msg2.0, msg2.1);

    let msg = motor1.build_send_message(Command::build_switch_models(Openloop));
    motor1.send_messages(msg);

    let msg = motor2.build_send_message(Command::build_switch_models(Openloop));
    motor2.send_messages(msg);

    let msg = motor1.build_send_message(Command::build_askstatus);
    motor1.send_messages(msg);

    let msg = motor2.build_send_message(Command::build_askstatus);
    motor2.send_messages(msg);

    thread::sleep(Duration::from_secs(1));


    let msg1 = match motor1.recv_msg() {
        Some((return_type, msg)) => (return_type, msg),
        None => {
            println!("recv_msg None");
            (0, [0; 8])
        }
    };
    motor1.decode_msg(msg1.0, msg1.1);

    let msg2 = match motor2.recv_msg() {
        Some((return_type, msg)) => (return_type, msg),
        None => {
            println!("recv_msg None");
            (0, [0; 8])
        }
    };
    motor1.decode_msg(msg2.0, msg2.1);

    let msg = motor1.build_send_message(Command::build_speed(6.0));
    motor1.send_messages(msg);
    
    let msg = motor2.build_send_message(Command::build_speed(6.0));
    motor2.send_messages(msg);


    let msg = motor1.build_send_message(Command::build_openloop_voltage(1.0));
    motor1.send_messages(msg);
    
    let msg = motor2.build_send_message(Command::build_openloop_voltage(1.0));
    motor2.send_messages(msg);


    thread::sleep(Duration::from_secs(3));


    let msg = motor1.build_send_message(Command::build_openloop_voltage(0.0));
    motor1.send_messages(msg);
    
    let msg = motor2.build_send_message(Command::build_openloop_voltage(0.0));
    motor2.send_messages(msg);


    let msg = motor1.build_send_message(Command::build_askstatus);
    motor1.send_messages(msg);

    let msg = motor2.build_send_message(Command::build_askstatus);
    motor2.send_messages(msg);


    thread::sleep(Duration::from_secs(1));


    let msg1 = match motor1.recv_msg() {
        Some((return_type, msg)) => (return_type, msg),
        None => {
            println!("recv_msg None");
            (0, [0; 8])
        }
    };
    motor1.decode_msg(msg1.0, msg1.1);

    let msg2 = match motor2.recv_msg() {
        Some((return_type, msg)) => (return_type, msg),
        None => {
            println!("recv_msg None");
            (0, [0; 8])
        }
    };
    motor1.decode_msg(msg2.0, msg2.1);


    thread::sleep(Duration::from_secs(2));

    canbus1.canbus_close();
    println!("clse success!");

    let msg1 = match motor1.recv_msg() {
        Some((return_type, msg)) => (return_type, msg),
        None => {
            println!("recv_msg None");
            (0, [0; 8])
        }
    };
    motor1.decode_msg(msg1.0, msg1.1);

    let msg2 = match motor2.recv_msg() {
        Some((return_type, msg)) => (return_type, msg),
        None => {
            println!("recv_msg None");
            (0, [0; 8])
        }
    };
    motor1.decode_msg(msg2.0, msg2.1);
}
