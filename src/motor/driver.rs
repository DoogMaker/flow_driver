use std::sync::{Arc};
use socketcan::{CanFrame, EmbeddedFrame, StandardId};
use crate::can::transport::{CanBus, MotorMsg};
use crate::protocol::command::{self, ReturnMessage};

pub struct Motor {
    id: u32,
    can_bus: Arc<CanBus>,
}

impl Motor {
    pub fn new (id: u32, bus: Arc<CanBus>) -> Self {
        Self { 
            id,
            can_bus: Arc::clone(&bus),
        }
    }

    // build send messages 
    pub fn build_send_message(&self, cmd: command::Command) -> (u32, [u8; 8]) {
        let (id, data) = command::build_send_message(self.id, cmd);
        (id, data)
    }

    // send messages -> tx
    pub fn send_messages(&self, data: (u32, [u8; 8])){
        let _ = self.can_bus.send_tx.send(data);
    }

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

    pub fn decode_msg(&self, return_type: u32, msg: [u8; 8]) -> Option<ReturnMessage> {
        if let Some((_, return_decode)) = command::decode_recv_message(return_type, msg) {
            Some(return_decode)
        } else {
            println!("decode Err");
            None
        }
    }

    pub fn print_msg(msg: Option<ReturnMessage>) {

        match msg {
            Some(ReturnMessage::FaultDetails(frame)) => println!("fault: {frame:?}"),
            Some(ReturnMessage::StatusFeedback(frame)) => println!("status: {frame:?}"),
            Some(ReturnMessage::ParameterResponse(frame)) => println!("parameter: {frame:?}"),
            None => println!("decode Err"),
        }
        
    }
}