use std::thread;
use std::time::{Duration,Instant};
use socketcan::{BlockingCan, CanFrame, CanSocket, EmbeddedFrame, Socket, StandardId};
use std::sync::mpsc::{Receiver, SyncSender, sync_channel, RecvTimeoutError};
use crate::protocol::command::{build_send_message, Command};



const ALIVE_TIMEOUT: u64 = 20; // 20ms
const SEND_TIMEOUT: u32 = 2; // 2ms
const INIT_ITER: Duration = Duration::from_millis(1); // 1ms

struct CanBus {
    tx: SyncSender<CanFrame>
}

impl CanBus {
    pub fn new(motor_start: u32, motor_stop: u32) -> Self {
        // create a channel for sending CAN frames
        let (tx, rx) = sync_channel::<CanFrame>(100);

        // move rx into send-thread
        thread::spawn(move || {
            send_thread(rx, motor_start, motor_stop, "can0");
        });

        Self { tx }
    }
}

//======================================

fn make_can_frame(id: u32, data: &[u8; 8])  -> CanFrame{
    let std_id = StandardId::new(id as u16).unwrap();
    let can_frame = CanFrame::new(std_id, data).unwrap();
    can_frame
}

struct MotorTimers {
    motor_id: u32,
    heartbeat_time: Instant,
    interval: Duration,
}

fn can_init(can_num: &str) -> std::io::Result<CanSocket> {
    CanSocket::open(can_num) // 这里传变量，不要加引号
}

fn send_can_frame(frame: &CanFrame, socket: &mut CanSocket) -> std::result::Result<(), socketcan::Error>{
    socket.transmit(frame)
}

fn send_can_keepalive_frame(node_id: u32, socket: &mut CanSocket) {
    let (id, msg) = build_send_message(node_id, Command::build_keepalive);
    let can_frame = make_can_frame(id, &msg);
    socket.transmit(&can_frame);
}

fn send_thread(rx: Receiver<CanFrame>, motor_start: u32, motor_stop: u32, can_num: &str) {
    // set keepalive timeout and init motor start time 
    //make differt motor has dirrerent init start time
    let alive_timeout: Duration = Duration::from_millis(
        ALIVE_TIMEOUT / (motor_stop - motor_start + 1) as u64
        );
    let base_time_start: Instant = Instant::now();

    let mut all_heartbeat_time: Vec<MotorTimers> = (motor_start..=motor_stop)
        .map(|id| MotorTimers {
            motor_id: id,
            heartbeat_time: base_time_start + INIT_ITER * id,
            interval: alive_timeout,
        })
        .collect();

    let mut socket1 = can_init(can_num)
        .expect("failed open can");

    loop {
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
                send_can_frame(&frame, &mut socket1);
                if let Some(motor) = all_heartbeat_time
                    .iter_mut()
                    .find( |motor| motor.motor_id == min_id) {
                        motor.heartbeat_time = Instant::now();
                    }
            },

            Err(RecvTimeoutError::Timeout) => {

            },

            Err(RecvTimeoutError::Disconnected) => {
                break;
            },
        };

        for timer in all_heartbeat_time.iter_mut() {
            if timer.heartbeat_time.elapsed() >= timer.interval {
                send_can_keepalive_frame(min_id, &mut socket1);
                timer.heartbeat_time = Instant::now();
            }
        }
    }
}

//======================================


