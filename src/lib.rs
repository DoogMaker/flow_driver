//! 基于 SocketCAN 的电机驱动库。
#![warn(missing_docs, unnameable_types)]

mod can;
mod motor;
mod protocol;

pub use can::transport::CanBus;
pub use motor::driver::Motor;

// 门面：命令、返回值、帧类型基址
pub use protocol::command::{
    Command, ReturnMessage,
    StatusFeedbackFrameLow, FaultDetailsFrameLow, ParameterResponseFrameLow,

};

// 让 Command 的每个变体都能被构造
pub use protocol::constants::{
    SwitchModelsByte1 as MotorMode,
    SetDirectionByte1 as Direction,
    GeneralParametersByte1 as ParamOpcode,
    GeneralParametersByte23 as ParamId,
};

// 输出类型：让外部能拿到结构化反馈
pub use protocol::decode::{
    StatusFeedbackFrame, FaultDetailsFrame, ParameterResponseFrame,
};
