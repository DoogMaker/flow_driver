use std::sync::{Arc};
use crate::can::transport::{CanBus, MotorMsg};
use crate::protocol::command::{self, ReturnMessage};
use crate::protocol::constants::AllFrameBasic;
use crate::motor_move::move_method;
use std::time::{self, Duration};
use std::thread;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};

/// Motor: motor class
/// id: motor id
/// can_bus: can bus
pub struct Motor {
    pub id: u32,
    pub can_bus: Arc<CanBus>,
}

impl Motor {
    /// init new motor
    /// id: motor id(you set)
    /// bus: can bus(need init)
    pub fn new (id: u32, bus: Arc<CanBus>) -> Self {
        Self {
            id,
            can_bus: Arc::clone(&bus),
        }
    }

    /// build send_messages
    /// input cmd: command
    /// return: (id, data)
    pub fn build_message(&self, cmd: command::Command) -> (u32, [u8; 8]) {
        let (id, data) = command::build_send_message(self.id, cmd);
        (id, data)
    }

    /// send messages -> tx
    /// input data: (id, data)
    pub fn send_messages(&self, data: (u32, [u8; 8])){
        let _ = self.can_bus.send_tx.send(data);
    }

    /// build and send send_msg
    /// include build_message and send_messages
    /// input cmd: command
    pub fn build_and_send_message(&self, cmd: command::Command) {
        let data = self.build_message(cmd);
        self.send_messages(data);
    }

    /// receive and decode and print msg
    pub fn rdp_msg(&self) {
        if let Some(msg) = self.recv_msg() {
            self.decode_msg(msg.0, msg.1);
        }
    }

    /// recv_msg to cache
    /// return: (return_type, msg)
    fn recv_msg(&self) -> Option<(u32, [u8; 8])>{
        let mut msg = self.can_bus.classed_msgs.lock().unwrap();

        msg
            .iter_mut()
            .find(|x| x.id == self.id)
            .and_then(
                |x: &mut MotorMsg| {
                    let return_type = x.return_type.pop_front()?;
                    let msg = x.data_queue.pop_front()?;
                    Some((return_type, msg))
                }
            )
    }

    /// print msg on bash
    /// input msg: ReturnMessage
    fn print_msg(msg: ReturnMessage) {

        match msg {
            ReturnMessage::FaultDetails(frame) => println!("fault: {frame:?}"),
            ReturnMessage::StatusFeedback(frame) => println!("status: {frame:?}"),
            ReturnMessage::ParameterResponse(frame) => println!("parameter: {frame:?}"),
        }

    }

    /// decode msg
    /// input return_type, msg
    fn decode_msg(&self, return_type: u32, msg: [u8; 8]) {
        if let Some((_, return_decode)) = command::decode_recv_message(return_type, msg) {
            Self::print_msg(return_decode);
        } else {
            println!("decode Err");
        }
    }



    ///get now rad
    pub fn get_rad(&self) -> Option<f32> {
        self.build_and_send_message(command::Command::BuildAskstatus);

        thread::sleep(Duration::from_secs_f32(0.1));

        let frame_type = AllFrameBasic::StatusFeedbackFrame as u32;

        let response = {
            let mut msg = self.can_bus.classed_msgs.lock().unwrap();

            msg.iter_mut()
                .find(|x| {
                    x.id == self.id
                        && x.return_type.front() == Some(&frame_type)
                })
                .and_then(|x: &mut MotorMsg| {
                    let return_type = x.return_type.pop_front()?;
                    let data = x.data_queue.pop_front()?;
                    Some((return_type, data))
                })
        };

        if let Some((return_type, data)) = response {
            if let Some((_, ReturnMessage::StatusFeedback(frame))) =
                command::decode_recv_message(return_type, data)
            {
                return Some(frame.rad);
            }
        }

        None
    }

    ///
   pub fn move_torad(&self, target_rad: f32) -> Result<(), &'static str> {
    let now_rad = self
        .get_rad()
        .ok_or("No rad return")?;

    let tf = 2.0;
    let v_min: f64 = target_rad as f64/50.0;

    let coeffs = move_method::equation_coefficients(
        now_rad as f64,
        target_rad as f64,
        v_min, 0.0,
        0.0, 0.0,
        tf,
    )?;

    let (time, position, _velocity, _acceleration) =
        move_method::equation_solution(coeffs, tf, 50)?;
        // println!("{:?}", &_acceleration);
        // println!{"====="};
        // println!("{:?}", &_velocity);
        // println!{"====="};
        // println!("{:?}", &position);

    let start_time = time::Instant::now();

    for (&t, &rad) in time.iter().zip(position.iter()) {
        let deadline = start_time + time::Duration::from_secs_f64(t);
        thread::sleep(deadline.saturating_duration_since(time::Instant::now()));
        self.build_and_send_message(command::Command::BuildState(rad as f32));
    }

    Ok(())
    }

}

pub struct Motorss {
    pub id_s: [Motor; 12],
    pub now_r: Arc<Mutex<[f64; 12]>>,
    // pub now_w: Arc<Mutex<[f64; 12]>>,
    running: Arc<AtomicBool>,
}

impl Motorss {
    pub fn Motorss_init() -> Self{
        let canbus1 = Arc::new(CanBus::canbus_init(1, 6, "can0".to_string()));
        let canbus2 = Arc::new(CanBus::canbus_init(7, 12, "can1".to_string()));

        let id_s: [Motor; 12] = [
            Motor::new(1,  Arc::clone(&canbus1)),
            Motor::new(2,  Arc::clone(&canbus1)),
            Motor::new(3,  Arc::clone(&canbus1)),
            Motor::new(4,  Arc::clone(&canbus1)),
            Motor::new(5,  Arc::clone(&canbus1)),
            Motor::new(6,  Arc::clone(&canbus1)),
            Motor::new(7,  Arc::clone(&canbus2)),
            Motor::new(8,  Arc::clone(&canbus2)),
            Motor::new(9,  Arc::clone(&canbus2)),
            Motor::new(10, Arc::clone(&canbus2)),
            Motor::new(11, Arc::clone(&canbus2)),
            Motor::new(12, Arc::clone(&canbus2)),
        ];

        let now_r: Arc<Mutex<[f64; 12]>> = Arc::new(Mutex::new([0f64; 12]));
        // let now_w: Arc<Mutex<[f64; 12]>> = Arc::new(Mutex::new([0f64; 12]));
        let running = Arc::new(AtomicBool::new(true));

        Motorss {
            id_s,
            now_r,
            // now_w,
            running,
        }
    }

    pub fn flash_status(&self) {
        while self.running.load(Ordering::Relaxed) {
            for (index, motor) in self.id_s.iter().enumerate() {
                match motor.get_rad() {
                    Some(rad) => {
                        let mut now_r = self.now_r.lock().unwrap();
                        now_r[index] = rad as f64;
                    }
                    None => {
                        eprintln!("motor {} rad read failed", motor.id);
                    }
                }
            }
        }
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::Relaxed);
    }
}
