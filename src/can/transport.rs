use std::thread;
use std::time::{Duration,Instant};
use socketcan::{BlockingCan, CanFrame, CanSocket, EmbeddedFrame, Socket, StandardId, Frame};
use std::sync::mpsc::{Receiver, RecvTimeoutError, SyncSender, sync_channel};
use crate::protocol::command::{build_send_message, Command, filter_frame_id};
use super::constants::{ALIVE_TIMEOUT, SEND_TIMEOUT, INIT_ITER};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

pub struct CanBus {
    pub send_tx: SyncSender<CanFrame>,
    pub classed_msgs: Arc<Mutex<Vec<MotorMsg>>>,
}

impl CanBus {
    pub fn canbus_init(motor_start: u32, motor_stop: u32, can_num: String) -> Self {
        // create a channel for sending CAN frames
        let mut send_socket = can_init(&can_num)
            .expect("failed open can");

        let mut recv_socket = can_init(&can_num)
            .expect("failed open can");

        let (send_tx, send_rx) = sync_channel::<CanFrame>(100);

        let classedmsgs: Arc<Mutex<Vec<MotorMsg>>> = Arc::new(Mutex::new((motor_start..=motor_stop)
            .map (
                |id: u32| MotorMsg {
                    id,
                    return_type: VecDeque::new(),
                    data_queue: VecDeque::new(),
                }
            )
            .collect()
        ));

        // move rx into send-thread
        thread::spawn(move || {
            send_thread(send_rx, motor_start, motor_stop, &mut send_socket);
        });

        let classed_msgs: Arc<Mutex<Vec<MotorMsg>>> = Arc::clone(&classedmsgs);
        thread::spawn(
            move || {
                recv_thread(classed_msgs, &mut recv_socket);
        });

        Self { 
            send_tx, 
            classed_msgs: classedmsgs,
        }
    }
}

//======================================

struct MotorTimers {
    motor_id: u32,
    heartbeat_time: Instant,
    interval: Duration,
}

fn make_can_frame(id: u32, data: &[u8; 8])  -> CanFrame{
    let std_id = StandardId::new(id as u16).unwrap();
    let can_frame = CanFrame::new(std_id, data).unwrap();
    can_frame
}

fn can_init(can_num: &str) -> std::io::Result<CanSocket> {
    CanSocket::open(can_num)
}

fn send_can_frame(frame: &CanFrame, socket: &mut CanSocket) -> std::result::Result<(), socketcan::Error>{
    socket.transmit(frame)
}

fn send_can_keepalive_frame(node_id: u32, socket: &mut CanSocket) {
    let (id, msg) = build_send_message(node_id, Command::build_keepalive);
    let can_frame = make_can_frame(id, &msg);
    socket.transmit(&can_frame);
}
//=====================================

pub struct MotorMsg {
    pub id: u32,
    pub return_type: VecDeque<u32>,
    pub data_queue: VecDeque<[u8; 8]>,
}

fn recv_can_frame(
    socket: &mut CanSocket
) -> (u32, u32, [u8; 8]) {

    let can_frame: CanFrame = socket.read_frame()
        .expect("read Err");
    let (return_type, id) = filter_frame_id(can_frame.raw_id() as u16);
    let mut data = [0u8; 8];
    data.copy_from_slice(can_frame.data());
    (id as u32, return_type as u32, data)
}


//=====================================

fn send_thread(
    rx: Receiver<CanFrame>,
    motor_start: u32,
    motor_stop: u32,
    socket1: &mut CanSocket) {

    // set keepalive timeout and init motor start time 
    // make differt motor has dirrerent init start time
    let alive_timeout: Duration = Duration::from_millis(
        ALIVE_TIMEOUT
        );
    let base_time_start: Instant = Instant::now();

    let mut all_heartbeat_time: Vec<MotorTimers> = (motor_start..=motor_stop)
        .map(|id| MotorTimers {
            motor_id: id,
            heartbeat_time: base_time_start + INIT_ITER * id,
            interval: alive_timeout,
        })
        .collect();

    loop {

        thread::sleep(SEND_TIMEOUT);

        let min_id_and_timeout = all_heartbeat_time.iter()
            .map(
                |timer|{(
                    timer.motor_id,
                    timer.interval.saturating_sub(timer.heartbeat_time.elapsed()),
                )}
            )
            .min_by_key(|(_, timeout)| * timeout);

        let (min_id, min_timeout) = min_id_and_timeout.expect("motor init Err");

        match rx.recv_timeout(min_timeout) {
            Ok(frame) => {
                send_can_frame(&frame, socket1).unwrap();
                if let Some(motor) = all_heartbeat_time
                    .iter_mut()
                    .find( |motor| motor.motor_id == min_id) {
                        motor.heartbeat_time = Instant::now();
                    }
            },

            Err(RecvTimeoutError::Timeout) => {},

            Err(RecvTimeoutError::Disconnected) => {break;},
        };

        for timer in all_heartbeat_time.iter_mut() {
            if timer.heartbeat_time.elapsed() >= timer.interval {
                send_can_keepalive_frame(min_id, socket1);
                timer.heartbeat_time = Instant::now();
            }
        }
    }
}

//======================================



fn recv_thread(
    classed_msgs: Arc<Mutex<Vec<MotorMsg>>>,
    socket1: &mut CanSocket) {

    loop {
        let (id,return_type, data) = recv_can_frame(socket1);

        println!("Recv a msg, ID: {id}, Type: {return_type}");

        let mut classed_msgs = classed_msgs.lock().unwrap();

        let classed_msg = classed_msgs
            .iter_mut()
            .find(|x| x.id == id) 
            .map(|x| {
                x.data_queue.push_back(data);
                x.return_type.push_back(return_type);
            })                    
            .ok_or("unknown id");
    }

}