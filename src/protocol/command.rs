use super::constants::*;
use super::decode::*;
use super::encode::*;
pub(crate) const STATUS_FEEDBACK_FRAME_LOW: u16 = AllFrameBasic::StatusFeedbackFrame as u16;
pub(crate) const STATUS_FEEDBACK_FRAME_HEIGH: u16 = STATUS_FEEDBACK_FRAME_LOW + MAX_NODE_NUMBER as u16 + 1;

pub(crate) const FAULT_DETAILS_FRAME_LOW: u16 = AllFrameBasic::FaultDetailsFrame as u16;
pub(crate) const FAULT_DETAILS_FRAME_HEIGH: u16 = FAULT_DETAILS_FRAME_LOW + MAX_NODE_NUMBER as u16 + 1;

pub(crate) const PARAMETER_RESPONSE_FRAME_LOW: u16 = AllFrameBasic::ParameterResponseFrame as u16;
pub(crate) const PARAMETER_RESPONSE_FRAME_HEIGH: u16 = PARAMETER_RESPONSE_FRAME_LOW + MAX_NODE_NUMBER as u16 + 1;

/// CMD
pub enum CMD
{
    /// set speed
    BuildSpeed(f32),
    /// set openloop voltage
    BuildOpenloopVoltage(f32),
    /// set switch models
    BuildSwitchModels(Models),
    /// set direction
    BuildSetDirection(Direction),
    /// set torque
    BuildTorque(f32),
    /// set state
    BuildState(f32),
    /// keepalive
    BuildKeepalive,
    /// ask status
    BuildAskstatus,
    /// set nodenum
    BuildSetNodenum(u8),
    /// build general parameters
    BuildGeneralParameters(
        ParamAction,
        ParamFaulfID,
        Option<f32>),
}

pub fn build_msg(node_id: u32, cmd: CMD) -> (u32, [u8; 8])
{
    let frame_id: u32 = make_frame(node_id);
    let data: [u8; 8] = match cmd
    {
        CMD::BuildSpeed(val) =>
            build_speed(val),
        CMD::BuildOpenloopVoltage(val) =>
            build_openloop_voltage(val),
        CMD::BuildSwitchModels(model) =>
            build_switch_models(model),
        CMD::BuildSetDirection(direction) =>
            build_set_direction(direction),
        CMD::BuildTorque(val) =>
            build_torque(val),
        CMD::BuildState(val) =>
            build_state(val),
        CMD::BuildKeepalive =>
            build_keepalive(),
        CMD::BuildAskstatus =>
            build_askstatus(),
        CMD::BuildSetNodenum(val) =>
            build_set_nodenum(val),
        CMD::BuildGeneralParameters(opcode, id, val) =>
            build_general_parameters(opcode, Some(id), val),
    };
    (frame_id, data)
}

pub fn filter_frame_id(frame: u16) -> (u16, u16)
{
    let (result, node_id) = match frame
    {
        STATUS_FEEDBACK_FRAME_LOW..STATUS_FEEDBACK_FRAME_HEIGH =>
            (STATUS_FEEDBACK_FRAME_LOW, frame - STATUS_FEEDBACK_FRAME_LOW),

        FAULT_DETAILS_FRAME_LOW..FAULT_DETAILS_FRAME_HEIGH =>
            (FAULT_DETAILS_FRAME_LOW, frame - FAULT_DETAILS_FRAME_LOW),

        PARAMETER_RESPONSE_FRAME_LOW..PARAMETER_RESPONSE_FRAME_HEIGH =>
            (PARAMETER_RESPONSE_FRAME_LOW, frame - PARAMETER_RESPONSE_FRAME_LOW),
        
        _ => (0, 0),
    };
    (result, node_id)
}

fn match_status_feedback_frame(data: [u8; 8]) -> Option<StatusFeedbackFrame>
{
    let result = status_feedback_frame(&data);
    result
}

fn match_fault_details_frame(data: [u8; 8]) -> Option<FaultDetailsFrame>
{
    let result = fault_details_frame(&data);
    result
}

fn match_parameter_response_frame(data: [u8; 8]) -> Option<ParameterResponseFrame>
{
    let result = parameter_response_frame(&data);
    result
}

/// ReturnMessage
#[derive(Debug)]
pub enum ReturnMessage {
    /// StatusFeedbackFrame
    StatusFeedback(StatusFeedbackFrame),
    /// FaultDetailsFrame
    FaultDetails(FaultDetailsFrame),
    /// ParameterResponseFrame
    ParameterResponse(ParameterResponseFrame),
}

pub fn decode_recv_message(return_type: u32, data: [u8; 8]) -> Option<(u32, ReturnMessage)>
{
    let message = match return_type as u16
    {
        FAULT_DETAILS_FRAME_LOW =>
            {match_fault_details_frame(data).map(ReturnMessage::FaultDetails)},
        STATUS_FEEDBACK_FRAME_LOW =>
            {match_status_feedback_frame(data).map(ReturnMessage::StatusFeedback)},
        PARAMETER_RESPONSE_FRAME_LOW =>
            {match_parameter_response_frame(data).map(ReturnMessage::ParameterResponse)},
        _ => None,
    };
    message.map(|message| (return_type, message))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_code() {
        let result = build_msg(1, CMD::BuildSpeed(10.0));
        assert!(result.0 == 0x101);
        assert!(result.1 == [0x01, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_msg(2, CMD::BuildOpenloopVoltage(0.6));
        assert!(result.0 == 0x102);
        assert!(result.1 == [0x02, 0x3C, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_msg(3, CMD::BuildSwitchModels(Models::Openloop));
        assert!(result.0 == 0x103);
        assert!(result.1 == [0x03, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_msg(4, CMD::BuildSwitchModels(Models::IDLE));
        assert!(result.0 == 0x104);
        assert!(result.1 == [0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_msg(5, CMD::BuildSetDirection(Direction::BackwardRotation));
        assert!(result.0 == 0x105);
        assert!(result.1 == [0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_msg(6, CMD::BuildTorque(1.0));
        assert!(result.0 == 0x106);
        assert!(result.1 == [0x05, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_msg(7, CMD::BuildState(1.0));
        assert!(result.0 == 0x107);
        assert!(result.1 == [0x06, 0x10, 0x27, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_msg(8, CMD::BuildKeepalive);
        assert!(result.0 == 0x108);
        assert!(result.1 == [0x07, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_msg(9, CMD::BuildAskstatus);
        assert!(result.0 == 0x109);
        assert!(result.1 == [0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_msg(10, CMD::BuildSetNodenum(3));
        assert!(result.0 == 0x10A);
        assert!(result.1 == [0x09, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_msg(11, CMD::BuildGeneralParameters(ParamAction::ReadEffectiveval, ParamFaulfID::SpeedLoopKp, None));
        assert!(result.0 == 0x10B);
        assert!(result.1 == [0x0A, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]);

        let result = build_msg(12, CMD::BuildGeneralParameters(ParamAction::WriteTemporaryval, ParamFaulfID::SpeedLoopKp, Some(0.1)));
        assert!(result.0 == 0x10C);
        assert!(result.1 == [0x0A, 0x02, 0x00, 0x01, 0xCD, 0xCC, 0xCC, 0x3D]);

        let result = build_msg(1, CMD::BuildGeneralParameters(ParamAction::WriteTemporaryval, ParamFaulfID::PoseloopKp, Some(0.5)));
        assert!(result.0 == 0x101);
        assert!(result.1 == [0x0A, 0x02, 0x09, 0x01, 0x00, 0x00, 0x00, 0x3F]);

        let result = build_msg(2, CMD::BuildGeneralParameters(ParamAction::WriteTemporaryval, ParamFaulfID::FeedforwardVelGain, Some(1.0)));
        assert!(result.0 == 0x102);
        assert!(result.1 == [0x0A, 0x02, 0x0C, 0x01, 0x00, 0x00, 0x80, 0x3F]);

        let result = build_msg(3, CMD::BuildGeneralParameters(ParamAction::SubmitEffectiveval, ParamFaulfID::None, None));
        assert!(result.0 == 0x103);
        assert!(result.1 == [0x0A, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_msg(4, CMD::BuildGeneralParameters(ParamAction::SaveEffectiveval, ParamFaulfID::None, None));
        assert!(result.0 == 0x104);
        assert!(result.1 == [0x0A, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);

        let result = build_msg(5, CMD::BuildGeneralParameters(ParamAction::ResetDefault, ParamFaulfID::None, None));
        assert!(result.0 == 0x105);
        assert!(result.1 == [0x0A, 0x05, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    }

    #[test]
    fn test_decode_message() {
        let frame = PARAMETER_RESPONSE_FRAME_LOW + 3;
        let response_data = [
            [0x80, 0x00, 0x00, 0x01, 0xCD, 0xCC, 0xCC, 0x3D],
            [0x82, 0x05, 0x00, 0x01, 0x00, 0x00, 0xFA, 0x45],
            [0x83, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            [0x84, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
            [0x85, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
        ];

        for data in response_data {
            let result = decode_recv_message(frame as u32, data);
            assert!(
                matches!(result, Some((3, ReturnMessage::ParameterResponse(_)))),
                "expected parameter response for data {data:02X?}"
            );
        }
    }
}
