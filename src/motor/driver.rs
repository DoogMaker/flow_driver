use std::sync::{Arc};
use socketcan::CanFrame;
use crate::can::transport::{CanBus, MotorMsg, make_can_frame};
use crate::protocol::command::{ReturnMessage};
use crate::protocol::command;

/// Motor: motor class
/// id: motor id
/// can_bus: can bus
pub struct Motor {
    /// Motor's id
    pub id: u32,
    /// Motor's id
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

    /// build canframe
    /// input CMD: CMD
    /// return CanFrame
    pub fn canframe_make(&self, cmd:command::CMD) -> CanFrame {
        let (id, data) = self.build_message(cmd);
        make_can_frame(id, &data)
    }

    /// build and send send_msg
    /// include build_message and send_messages
    /// input CMD: CMD
    pub fn build_send_message(&self, cmd: command::CMD) {
        let data = self.build_message(cmd);
        self.send_messages(data);
    }

    // build send_messages
    // input CMD: CMD
    // return: (id, data)
    fn build_message(&self, cmd: command::CMD) -> (u32, [u8; 8]) {
        let (id, data) = command::build_msg(self.id, cmd);
        (id, data)
    }

    // send messages -> tx
    // input data: (id, data)
    fn send_messages(&self, data: (u32, [u8; 8])){
        let _ = self.can_bus.send_tx.send(data);
    }

    /// receive and decode and print msg
    pub fn rdp_msg(&self) {
        loop {
            if let Some(msg) = self.recv_msg() {
                self.decode_msg(msg.0, msg.1, msg.2);
            } else {
                break;
            }
        }
    }

    /// recv_msg to cache
    /// return: (return_type, msg)
    fn recv_msg(&self) -> Option<(u32, u32, [u8; 8])>{
        let mut msg = self.can_bus.classed_msgs.lock().unwrap();

        msg.iter_mut()
            .find(|x| x.id == self.id)
            .and_then(
                |x: &mut MotorMsg| {
                    let id = x.id;
                    let return_type = x.return_type.pop_front()?;
                    let msg = x.data_queue.pop_front()?;
                    Some((id, return_type, msg))
                }
            )
    }

    /// print msg on bash
    /// input msg: ReturnMessage
    fn print_msg(id: u32, msg: ReturnMessage) {
        match msg {
            ReturnMessage::FaultDetails(frame) => println!("ID: {id} fault: {frame:?}"),
            ReturnMessage::StatusFeedback(frame) => println!("ID: {id} status: {frame:?}"),
            ReturnMessage::ParameterResponse(frame) => println!("ID: {id} parameter: {frame:?}"),
        }
    }

    /// decode msg
    /// input return_type, msg
    fn decode_msg(&self, id: u32, return_type: u32, msg: [u8; 8]) {
        if let Some((_, return_decode)) = command::decode_recv_message(return_type, msg) {
            Self::print_msg(id, return_decode);
        } else {
            println!("decode Err");
        };
    }
}
