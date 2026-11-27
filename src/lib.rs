//! 基于 SocketCAN 的电机驱动库。
#![warn(missing_docs, unnameable_types)]

mod can;
///
pub mod motor;
mod protocol;

pub use can::transport::CanBus;
pub use motor::driver::Motor;

pub use protocol::command::{
    CMD, ReturnMessage,

};

pub use protocol::constants::{
    Models,
    Direction,
    ParamAction,
    ParamFaultID,
};

pub use protocol::decode::{
    StatusFeedbackFrame, FaultDetailsFrame, ParameterResponseFrame,
};
