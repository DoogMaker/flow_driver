use super::constants::*;
use super::decode::*;
use super::encode::*;


pub const StatusFeedbackFrameLow: u16 = AllFrameBasic::StatusFeedbackFrame as u16;
pub const StatusFeedbackFrameHeigh: u16 = StatusFeedbackFrameLow + MAX_NODE_NUMBER as u16;

pub const FaultDetailsFrameLow: u16 = AllFrameBasic::FaultDetailsFrame as u16;
pub const FaultDetailsFrameHeigh: u16 = FaultDetailsFrameLow + MAX_NODE_NUMBER as u16;

pub const ParameterResponseFrameLow: u16 = AllFrameBasic::ParameterResponseFrame as u16;
pub const ParameterResponseFrameHeigh: u16 = ParameterResponseFrameLow + MAX_NODE_NUMBER as u16;

pub type Models = SwitchModelsByte1;
pub type Direction = SetDirectionByte1;
pub type Opcode = GeneralParametersByte1;
pub type Id = Option<GeneralParametersByte23>;

pub enum Command
{
    build_speed(f32),
    build_openloop_voltage(f32),
    build_switch_models(Models),
    build_set_direction(Direction),
    build_torque(f32),
    build_state(f32),
    build_keepalive,
    build_askstatus,
    build_set_nodenum(u8),
    build_general_parameters(Opcode,
        Id,
        Option<f32>),
}
pub fn build_send_message(node_id: u8, command: Command) -> (u16, [u8; 8])
{
    let frame_id: u16 = command_frame_id(node_id);
    let data: [u8; 8] = match command
    {
        Command::build_speed(val) =>
            build_speed(val),
        Command::build_openloop_voltage(val) =>
            build_openloop_voltage(val),
        Command::build_switch_models(model) =>
            build_switch_models(model),
        Command::build_set_direction(direction) =>
            build_set_direction(direction),
        Command::build_torque(val) =>
            build_torque(val),
        Command::build_state(val) =>
            build_state(val),
        Command::build_keepalive =>
            build_keepalive(),
        Command::build_askstatus =>
            build_askstatus(),
        Command::build_set_nodenum(val) =>
            build_set_nodenum(val),
        Command::build_general_parameters(opcode, id, val) =>
            build_general_parameters(opcode, id, val),
    };
    (frame_id, data)
}

fn filter_frame_id(frame: u16) -> (u16, u16)
{
    let (result, node_id) = match frame
    {
        StatusFeedbackFrameLow..StatusFeedbackFrameHeigh =>
            (StatusFeedbackFrameLow, frame - StatusFeedbackFrameLow),

        FaultDetailsFrameLow..FaultDetailsFrameHeigh =>
            (FaultDetailsFrameLow, frame - FaultDetailsFrameLow),

        ParameterResponseFrameLow..ParameterResponseFrameHeigh =>
            (ParameterResponseFrameLow, frame - ParameterResponseFrameLow),
        
        _ => (0, 0),
    };
    (result, node_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_code() {
        let result = build_send_message(1, Command::build_speed(10.0));
        assert!(result.0 == 0x101);
        assert!(result.1 == [0x01, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_send_message(2, Command::build_openloop_voltage(0.6));
        assert!(result.0 == 0x102);
        assert!(result.1 == [0x02, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_send_message(3, Command::build_switch_models(Models::Openloop));
        assert!(result.0 == 0x103);
        assert!(result.1 == [0x03, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_send_message(4, Command::build_switch_models(Models::IDLE));
        assert!(result.0 == 0x104);
        assert!(result.1 == [0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_send_message(5, Command::build_set_direction(Direction::BackwardRotation));
        assert!(result.0 == 0x105);
        assert!(result.1 == [0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_send_message(6, Command::build_torque(1.0));
        assert!(result.0 == 0x106);
        assert!(result.1 == [0x05, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_send_message(7, Command::build_state(1.0));
        assert!(result.0 == 0x107);
        assert!(result.1 == [0x06, 0x10, 0x27, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_send_message(8, Command::build_keepalive);
        assert!(result.0 == 0x108);
        assert!(result.1 == [0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_send_message(9, Command::build_askstatus);
        assert!(result.0 == 0x109);
        assert!(result.1 == [0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_send_message(10, Command::build_set_nodenum(3));
        assert!(result.0 == 0x10A);
        assert!(result.1 == [0x09, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_send_message(11, Command::build_general_parameters(Opcode::ReadEffectiveval, Some(GeneralParametersByte23::SpeedLoopKp), None));
        assert!(result.0 == 0x10B);
        assert!(result.1 == [0x0A, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]);

        let result = build_send_message(12, Command::build_general_parameters(Opcode::WriteTemporaryval, Some(GeneralParametersByte23::SpeedLoopKp), Some(0.1)));
        assert!(result.0 == 0x10C);
        assert!(result.1 == [0x0A, 0x02, 0x00, 0x01, 0xCD, 0xCC, 0xCC, 0x3D]);

        let result = build_send_message(1, Command::build_general_parameters(Opcode::WriteTemporaryval, Some(GeneralParametersByte23::PoseloopKp), Some(0.5)));
        assert!(result.0 == 0x101);
        assert!(result.1 == [0x0A, 0x02, 0x09, 0x01, 0x00, 0x00, 0x00, 0x3F]);

        let result = build_send_message(2, Command::build_general_parameters(Opcode::WriteTemporaryval, Some(GeneralParametersByte23::FeedforwardVelGain), Some(1.0)));
        assert!(result.0 == 0x102);
        assert!(result.1 == [0x0A, 0x02, 0x0C, 0x01, 0x00, 0x00, 0x80, 0x3F]);

        let result = build_send_message(3, Command::build_general_parameters(Opcode::SubmitEffectiveval, None, None));
        assert!(result.0 == 0x103);
        assert!(result.1 == [0x0A, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_send_message(4, Command::build_general_parameters(Opcode::SaveEffectiveval, None, None));
        assert!(result.0 == 0x104);
        assert!(result.1 == [0x0A, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_send_message(5, Command::build_general_parameters(Opcode::ResetDefault, None, None));
        assert!(result.0 == 0x105);
        assert!(result.1 == [0x0A, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    }
}
