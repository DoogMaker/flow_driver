use std::thread::{self, JoinHandle};
use std::time::{Duration,Instant};
use std::collections::VecDeque;
use std::io::Result;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, SyncSender, sync_channel};

use socketcan::{BlockingCan, CanFrame, CanSocket, EmbeddedFrame, Socket, StandardId, Frame};

use crate::protocol::command::{build_send_message, Command, filter_frame_id};

use super::constants::{ALIVE_TIMEOUT, SEND_TIMEOUT, INIT_ITER, RECV_TIMEOUT};


pub struct CanBus {
    pub send_tx: SyncSender<(u32, [u8; 8])>,
    pub classed_msgs: Arc<Mutex<Vec<MotorMsg>>>,
    pub stop_flags: Arc<AtomicBool>,
    send_handle: Mutex<Option<JoinHandle<()>>>,
    recv_handle: Mutex<Option<JoinHandle<()>>>,
}

impl CanBus {
    pub fn canbus_init(motor_start: u32, motor_stop: u32, can_num: String) -> Self {
        // create a channel for sending CAN frames
        let mut send_socket = can_init(&can_num)
            .expect("failed open can");

        let mut recv_socket = can_init(&can_num)
            .expect("failed open can");

        let (send_tx, send_rx) = sync_channel::<(u32, [u8; 8])>(100);

        let stop_flags: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

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
        let thread_stop_flags = Arc::clone(&stop_flags);
        let send_handle: JoinHandle<()> = thread::spawn(
            move || {
            send_thread(send_rx, motor_start, motor_stop, &mut send_socket, thread_stop_flags);
        });

        let thread_stop_flags = Arc::clone(&stop_flags);
        let classed_msgs: Arc<Mutex<Vec<MotorMsg>>> = Arc::clone(&classedmsgs);
        let recv_handle = thread::spawn(
            move || {
                recv_thread(classed_msgs, &mut recv_socket, thread_stop_flags);
        });

        Self { 
            send_tx, 
            classed_msgs: classedmsgs,
            stop_flags,
            send_handle: Mutex::new(Some(send_handle)),
            recv_handle: Mutex::new(Some(recv_handle)),
        }
    }

    pub fn canbus_close(&self) {
        self.stop_flags.store(true, Ordering::SeqCst);

        let mut send_handle = self.send_handle.lock().unwrap();
        let mut recv_handle = self.recv_handle.lock().unwrap();

        if let Some(handle) = send_handle.take() {
            handle.join().unwrap();
        }

        if let Some(handle) = recv_handle.take() {
            handle.join().unwrap();
        }
        println!("All thread has closed")
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
) -> Result<(u32, u32, [u8; 8])> {

    let can_frame: CanFrame = socket.read_frame_timeout(RECV_TIMEOUT)?;
    let (return_type, id) = filter_frame_id(can_frame.raw_id() as u16);
    let mut data = [0u8; 8];
    data.copy_from_slice(can_frame.data());
    Ok((id as u32, return_type as u32, data))
}


//=====================================

fn send_thread(
    rx: Receiver<(u32, [u8; 8])>,
    motor_start: u32,
    motor_stop: u32,
    socket1: &mut CanSocket,
    stop_flags: Arc<AtomicBool>) {

    // set keepalive timeout and init motor start time 
    // make differt motor has dirrerent init start time
    let alive_timeout: Duration = Duration::from_millis(
        ALIVE_TIMEOUT
        );
    let base_time_start: Instant = Instant::now();

    let mut all_heartbeat_time: Vec<MotorTimers> = (motor_start..=motor_stop)
        .map(|id| MotorTimers {
            motor_id: id,
            heartbeat_time: base_time_start - INIT_ITER * id,
            interval: alive_timeout,
        })
        .collect();

    loop {
        if stop_flags.load(Ordering::SeqCst) {
            break;
        }
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
            Ok((id, data)) => {
                let frame: CanFrame = make_can_frame(id, &data);
                send_can_frame(&frame, socket1).unwrap();
                if let Some(motor) = all_heartbeat_time
                    .iter_mut()
                    .find(|motor| motor.motor_id == min_id) {
                        motor.heartbeat_time = Instant::now();
                    }
            },

            Err(RecvTimeoutError::Timeout) => {},

            Err(RecvTimeoutError::Disconnected) => {break;},
        };

        for timer in all_heartbeat_time.iter_mut() {
            if timer.heartbeat_time.elapsed() >= timer.interval {
                send_can_keepalive_frame(timer.motor_id, socket1);
                timer.heartbeat_time = Instant::now();
            }
        }
    }
    drop(socket1);
}

//======================================



fn recv_thread(
    classed_msgs: Arc<Mutex<Vec<MotorMsg>>>,
    socket1: &mut CanSocket,
    stop_flags: Arc<AtomicBool>) {

    loop {
        if stop_flags.load(Ordering::SeqCst) {
            break;
        }
        let (id,return_type, data) = match recv_can_frame(socket1) {
            Ok(frame) => frame,
            Err(error) if error.kind() == std::io::ErrorKind::TimedOut => {
                // thread::sleep(Duration::from_millis(10));
                continue;
            }
            Err(error) => {
                eprintln!("CAN receive failed: {error}");
                break;
            }
        };

        if id != 0 {

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
    drop(socket1);
}