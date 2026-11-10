use core::panic;

use crate::protocol::constants::GeneralParametersByte23::PageInvalidMaskA;

use super::constants::*;
use repr_cast::repr_cast;

pub fn command_frame_id(node_id: u8) -> u16
{
    if node_id > MAX_NODE_NUMBER
    {
        panic!("node_id must be less than 12");
    }
    AllFrameBasic::CommandFrame as u16 + node_id as u16
}

pub fn build_speed(val: f32) -> [u8; 8]
{
    let mut data: [u8; 8] = [0u8; 8];
    let speed: i16 = (val * TARGETSPEED).round() as i16;

    data[0] = EncodeCommand::TargetSpeed as u8;
    data[1..3].copy_from_slice(&speed.to_le_bytes());
    data
}

pub fn build_openloop_voltage(val: f32) -> [u8; 8]
{
    let mut data: [u8; 8] = [0u8; 8];
    let voltage: i16 = (val * OPENLOOPVOLTAGE).round() as i16;

    data[0] = EncodeCommand::OpenloopVoltage as u8;
    data[1..3].copy_from_slice(&voltage.to_le_bytes());
    data
}

pub fn build_switch_models(model: SwitchModelsByte1) -> [u8; 8]
{
    let mut data: [u8; 8] = [0u8; 8];
    data[0] = EncodeCommand::SwitchModels as u8;
    data[1] = model as u8;
    data
}

pub fn build_set_direction(direction: SetDirectionByte1) -> [u8; 8]
{
    let mut data: [u8; 8] = [0u8; 8];
    data[0] = EncodeCommand::SetDirection as u8;
    data[1] = direction as u8;
    data
}

pub fn build_torque(val: f32) -> [u8; 8]
{
    let mut data: [u8; 8] = [0u8; 8];
    let torque: i16 = (val * TARGETTORQUE).round() as i16;

    data[0] = EncodeCommand::TargetTorque as u8;
    data[1..3].copy_from_slice(&torque.to_le_bytes());
    data
}

pub fn build_state(val: f32) -> [u8; 8]
{
    let mut data: [u8; 8] = [0u8; 8];
    let state: i32 = (val * TARGETSTATE).round() as i32;

    data[0] = EncodeCommand::TargetState as u8;
    data[1..5].copy_from_slice(&state.to_le_bytes());
    data
}

pub fn build_keepalive() -> [u8; 8]
{
    let mut data: [u8; 8] = [0u8; 8];
    data[0] = EncodeCommand::KeepAlive as u8;
    data
}

pub fn build_askstatus() -> [u8; 8]
{
    let mut data: [u8; 8] = [0u8; 8];
    data[0] = EncodeCommand::AskStatusFeedback as u8;
    data
}

pub fn build_set_nodenum(node_number: u8) -> [u8; 8]
{
    let mut data: [u8; 8] = [0u8; 8];
    data[0] = EncodeCommand::SetNodeNumber as u8;
    data[1] = node_number;
    data
}

pub fn build_general_parameters(
    byte1: GeneralParametersByte1,
    byte23: Option<GeneralParametersByte23>,
    val: Option<f32>
) -> [u8; 8]
{
    let mut data: [u8; 8] = [0u8; 8];
    data[0] = EncodeCommand::GeneralParameters as u8;
    data[1] = byte1 as u8;

    if let Some(parameter) = byte23
    {
        data[2..4].copy_from_slice(&(parameter as u16).to_le_bytes());
    }

    if let Some(val1) = val
    {
        data[4..8].copy_from_slice(&val1.to_le_bytes());
    } 

    data
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_frame_id() {

        let result: u16 = command_frame_id(1);
        assert_eq!(result, 0x101);

        let result: u16 = command_frame_id(2);
        assert_eq!(result, 0x102);

        let result: u16 = command_frame_id(3);
        assert_eq!(result, 0x103);

        let result: u16 = command_frame_id(4);
        assert_eq!(result, 0x104);

        let result: u16 = command_frame_id(5);
        assert_eq!(result, 0x105);
    }

    #[test]
    fn test_buld_send_code() {
        let result = build_speed(10.0);
        assert!(result.to_vec() == vec![0x01, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_openloop_voltage(0.6);
        assert!(result.to_vec() == vec![0x02, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_switch_models(SwitchModelsByte1::Openloop);
        assert!(result.to_vec() == vec![0x03, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_switch_models(SwitchModelsByte1::IDLE);
        assert!(result.to_vec() == vec![0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_set_direction(SetDirectionByte1::BackwardRotation);
        assert!(result.to_vec() == vec![0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_torque(1.0);
        assert!(result.to_vec() == vec![0x05, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_state(1.0);
        assert!(result.to_vec() == vec![0x06, 0x10, 0x27, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_keepalive();
        assert!(result.to_vec() == vec![0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_askstatus();
        assert!(result.to_vec() == vec![0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_set_nodenum(3);
        assert!(result.to_vec() == vec![0x09, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_general_parameters(GeneralParametersByte1::ReadEffectiveval, Some(GeneralParametersByte23::SpeedLoopKp), None);
        assert!(result.to_vec() == vec![0x0A, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]);

        let result = build_general_parameters(GeneralParametersByte1::WriteTemporaryval, Some(GeneralParametersByte23::SpeedLoopKp), Some(0.1));
        assert!(result.to_vec() == vec![0x0A, 0x02, 0x00, 0x01, 0xCD, 0xCC, 0xCC, 0x3D]);

        let result = build_general_parameters(GeneralParametersByte1::WriteTemporaryval, Some(GeneralParametersByte23::PoseloopKp), Some(0.5));
        assert!(result.to_vec() == vec![0x0A, 0x02, 0x09, 0x01, 0x00, 0x00, 0x00, 0x3F]);

        let result = build_general_parameters(GeneralParametersByte1::WriteTemporaryval, Some(GeneralParametersByte23::FeedforwardVelGain), Some(1.0));
        assert!(result.to_vec() == vec![0x0A, 0x02, 0x0C, 0x01, 0x00, 0x00, 0x80, 0x3F]);

        let result = build_general_parameters(GeneralParametersByte1::SubmitEffectiveval, None, None);
        assert!(result.to_vec() == vec![0x0A, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_general_parameters(GeneralParametersByte1::SaveEffectiveval, None, None);
        assert!(result.to_vec() == vec![0x0A, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_general_parameters(GeneralParametersByte1::ResetDefault, None, None);
        assert!(result.to_vec() == vec![0x0A, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    }
}
