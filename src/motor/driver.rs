use std::sync::{Arc};
use crate::can::transport::{CanBus, MotorMsg};
use crate::protocol::command::{self, ReturnMessage};

/// Motor: motor class
/// id: motor id
/// can_bus: can bus
pub struct Motor {
    id: u32,
    can_bus: Arc<CanBus>,
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

    /// recv_msg to cache
    /// return: (return_type, msg)
    pub fn recv_msg(&self) -> Option<(u32, [u8; 8])>{
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
    pub fn print_msg(msg: ReturnMessage) {

        match msg {
            ReturnMessage::FaultDetails(frame) => println!("fault: {frame:?}"),
            ReturnMessage::StatusFeedback(frame) => println!("status: {frame:?}"),
            ReturnMessage::ParameterResponse(frame) => println!("parameter: {frame:?}"),
        }
        
    }

    /// decode msg
    /// input return_type, msg
    pub fn decode_msg(&self, return_type: u32, msg: [u8; 8]) {
        if let Some((_, return_decode)) = command::decode_recv_message(return_type, msg) {
            Self::print_msg(return_decode);
        } else {
            println!("decode Err");
        }
    }

    /// receive and decode and print msg
    pub fn rdp_msg(&self) {
        if let Some(msg) = self.recv_msg() {
            self.decode_msg(msg.0, msg.1);
        }
    }


}