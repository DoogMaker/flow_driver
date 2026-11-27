use std::collections::VecDeque;
use std::io::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{Receiver, RecvTimeoutError, SyncSender, sync_channel};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};
use socketcan::{BlockingCan, CanFrame, CanSocket, EmbeddedFrame, Frame, Socket, StandardId};
use crate::protocol::command::{CMD, build_msg, filter_frame_id};
use crate::protocol::constants::AllFrameBasic;
use super::constants::{ALIVE_TIMEOUT, INIT_ITER, RECV_TIMEOUT, SEND_TIMEOUT};
use crate::StatusFeedbackFrame;
use crate::protocol::constants::{self, Models, FeedBackStatus};
use crate::protocol::decode::status_feedback_frame;

/// can bus class
pub struct CanBus {
    /// send tx: main thread to send thread
    pub send_tx: SyncSender<(u32, [u8; 8])>,
    /// classed_msgs: recv thread to main thread
    pub(crate) classed_msgs: Arc<Mutex<Vec<MotorMsg>>>,
    pub(crate) now_status: Arc<Mutex<Vec<NowStatus>>>,

    stop_flags: Arc<AtomicBool>,
    send_handle: Mutex<Option<JoinHandle<()>>>,
    recv_handle: Mutex<Option<JoinHandle<()>>>,
}

impl CanBus {
    /// canbus_init
    /// input motor_start: motor start id, motor_stop: motor stop id, can_num: can bus number
    /// example:
    ///     mycanbus = canbus_init(1,1,"can1".to_string());
    ///     mycanbus = canbus_init(1,6,"can1".to_string());
    pub fn canbus_init(motor_start: u32, motor_stop: u32, can_num: String) -> Self {
        // create a channel for sending CAN frames
        let mut send_socket = can_init(&can_num).expect("failed open can");
        let mut recv_socket = can_init(&can_num).expect("failed open can");

        let (send_tx, send_rx) = sync_channel::<(u32, [u8; 8])>(100);

        let stop_flags: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

        let classedmsgs: Arc<Mutex<Vec<MotorMsg>>> = Arc::new(Mutex::new(
            (motor_start..=motor_stop)
                .map(|id: u32| MotorMsg {
                    id,
                    return_type: VecDeque::new(),
                    data_queue: VecDeque::new(),
                })
                .collect(),
        ));

        let now_status: Arc<Mutex<Vec<NowStatus>>> = Arc::new(Mutex::new(
            (motor_start..=motor_stop)
                .map(|id: u32| NowStatus {
                    id,
                    status: StatusFeedbackFrame {
                        system_state: FeedBackStatus::IDLE,
                        rotation_speed: 0f32,
                        current_a: 0f32,
                        model: Models::IDLE,
                        rad: 0f32 }
                })
                .collect(),
        ));

        // move rx into send-thread
        let thread_stop_flags = Arc::clone(&stop_flags);
        let send_handle: JoinHandle<()> = thread::spawn(move || {
            send_thread(
                send_rx,
                motor_start,
                motor_stop,
                &mut send_socket,
                thread_stop_flags,
            );
        });

        let thread_stop_flags = Arc::clone(&stop_flags);
        let classed_msgs: Arc<Mutex<Vec<MotorMsg>>> = Arc::clone(&classedmsgs);
        let now_status_thread: Arc<Mutex<Vec<NowStatus>>> = Arc::clone(&now_status);
        let recv_handle = thread::spawn(move || {
            recv_thread(classed_msgs,now_status_thread, &mut recv_socket, thread_stop_flags);
        });

        Self {
            send_tx,
            classed_msgs: classedmsgs,
            now_status,
            stop_flags,
            send_handle: Mutex::new(Some(send_handle)),
            recv_handle: Mutex::new(Some(recv_handle)),
        }
    }

    /// can bus close
    /// if success
    /// println!("All thread has closed")
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

pub(crate) fn make_can_frame(id: u32, data: &[u8; 8]) -> CanFrame {
    let std_id = StandardId::new(id as u16).unwrap();
    let can_frame = CanFrame::new(std_id, data).unwrap();
    can_frame
}

fn can_init(can_num: &str) -> std::io::Result<CanSocket> {
    CanSocket::open(can_num)
}

fn send_can_frame(
    frame: &CanFrame,
    socket: &mut CanSocket,
) -> std::result::Result<(), socketcan::Error> {
    socket.transmit(frame)
}

fn send_askstatus_canframe(node_id: u32, socket: &mut CanSocket) {
    let (id, msg) = build_msg(node_id, CMD::BuildAskstatus);
    let can_frame = make_can_frame(id, &msg);
    if let Err(e) = socket.transmit(&can_frame) {
        eprint!("keepalive send error: {e}")
    }
}
//=====================================

/// MotorMsg: motor msg class
/// id: motor id
/// return_type: return type
/// data_queue: data queue
pub(crate) struct MotorMsg {
    pub id: u32,
    pub return_type: VecDeque<u32>,
    pub data_queue: VecDeque<[u8; 8]>,
}

pub(crate) struct NowStatus {
    pub id: u32,
    pub status: StatusFeedbackFrame,
}

fn recv_can_frame(socket: &mut CanSocket) -> Result<(u32, u32, [u8; 8])> {
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
    stop_flags: Arc<AtomicBool>,
) {
    // set keepalive timeout and init motor start time
    // make different motor has dirrerent init start time
    let alive_timeout: Duration = Duration::from_millis(ALIVE_TIMEOUT);
    let base_time_start: Instant = Instant::now();

    let mut all_heartbeat_time: Vec<MotorTimers> = (motor_start..=motor_stop)
        .map(|id| MotorTimers {
            motor_id: id,
            heartbeat_time: base_time_start - INIT_ITER * id,
            interval: alive_timeout,
        })
        .collect();

    loop {
        // check stop_flags
        if stop_flags.load(Ordering::SeqCst) {
            break;
        }
        thread::sleep(SEND_TIMEOUT);

        // get min timeout motor's id
        let min_id_and_timeout = all_heartbeat_time
            .iter()
            .map(|timer| {
                (
                    timer.motor_id,
                    timer
                        .interval
                        .saturating_sub(timer.heartbeat_time.elapsed()),
                )
            })
            .min_by_key(|(_, timeout)| *timeout);

        let (_, min_timeout) = min_id_and_timeout.expect("motor init Err");

        match rx.recv_timeout(min_timeout) {
            Ok((id, data)) => {
                let frame: CanFrame = make_can_frame(id, &data);
                send_can_frame(&frame, socket1).unwrap();
                let node_id = id.saturating_sub(AllFrameBasic::CMDFrame as u32);
                if let Some(motor) = all_heartbeat_time
                    .iter_mut()
                    .find(|motor| motor.motor_id == node_id)
                {
                    motor.heartbeat_time = Instant::now();
                }
            }
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => {
                break;
            }
        };

        for timer in all_heartbeat_time.iter_mut() {
            if timer.heartbeat_time.elapsed() >= timer.interval {
                send_askstatus_canframe(timer.motor_id, socket1);
                timer.heartbeat_time = Instant::now();
            }
        }
    }
}

//======================================

fn recv_thread(
    classed_msgs: Arc<Mutex<Vec<MotorMsg>>>,
    now_status: Arc<Mutex<Vec<NowStatus>>>,
    socket1: &mut CanSocket,
    stop_flags: Arc<AtomicBool>,
) {
    loop {
        if stop_flags.load(Ordering::SeqCst) {
            break;
        }

        let (id, return_type, data) = match recv_can_frame(socket1) {
            Ok(frame) => frame,
            Err(error) if error.kind() == std::io::ErrorKind::TimedOut => {
                continue;
            }
            Err(error) => {
                eprintln!("CAN receive failed: {error}");
                break;
            }
        };

        if id != 0 {
            // println!("Recv a msg, ID: {id}, Type: {return_type}");

            if return_type == constants::AllFrameBasic::StatusFeedbackFrame as u32 {

                let decoded_msg: StatusFeedbackFrame  = status_feedback_frame(&data).unwrap();

                let mut now_status = now_status.lock().unwrap();
                let _ = now_status
                    .iter_mut()
                    .find(|x| x.id == id)
                    .map(|x| {
                        x.status.system_state = decoded_msg.system_state;
                        x.status.model = decoded_msg.model;
                        x.status.current_a = decoded_msg.current_a;
                        x.status.rad = decoded_msg.rad;
                        x.status.rotation_speed = decoded_msg.rotation_speed;
                    })
                    .ok_or("err status flash");
            } else {

                let mut classed_msgs = classed_msgs.lock().unwrap();

                let _ = classed_msgs
                    .iter_mut()
                    .find(|x| x.id == id)
                    .map(|x| {
                        x.data_queue.push_back(data);
                        x.return_type.push_back(return_type);
                    })
                    .ok_or("unknown id");
            }
        }
    }
}
