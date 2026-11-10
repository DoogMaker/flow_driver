use super::constants::*;
use repr_cast::repr_cast;
use bitflags::bitflags;


pub struct StatusFeedbackFrame
{
    system_state: StatusFeedbackByte0,
    rotation_speed: f32,
    current_a: f32,
    model: StatusFeedbackByte5,
    bus_voltage: f32,

}

pub fn status_feedback_frame(data: &[u8]) -> Option<StatusFeedbackFrame>
{
    if data.len() < 8 {
        return None;
    }

    let Some(system_state) = StatusFeedbackByte0::from_repr(data[0]) else {return None};

    let rotation_speed = i16::from_le_bytes([data[1], data[2]]) as f32 / TARGETSPEED;

    let current_a = i16::from_le_bytes([data[3], data[4]]) as f32 / CURRENT_A;

    let Some(model) = StatusFeedbackByte5::from_repr(data[5]) else {return None};

    let bus_voltage = i16::from_le_bytes([data[6], data[7]]) as f32 / BUSVOLTAGE;

    Some(StatusFeedbackFrame {
        system_state,
        rotation_speed,
        current_a,
        model,
        bus_voltage,
    })
}

pub struct FaultDetailsFrame
{
    fault_code: FaultDetailsByte0,
    fault_mask_code: FaultDetailsByte23,
    drv8353_status1: Option<FaultDetailsByte45>,
    drv8353_status2: Option<FaultDetailsByte67>,
}

pub fn fault_details_frame(data: &[u8]) -> Option<FaultDetailsFrame>
{
    if data.len() < 8 {
        return None;
    }

    let Some(fault_code) = FaultDetailsByte0::from_repr(data[0]) else {return None};

    let fault_details: bool = data[1] == FaultDetailsByte1::VALIDSTATUS as u8;

    let fault_mask_code: FaultDetailsByte23 = FaultDetailsByte23::from_bits_truncate(u16::from_le_bytes([data[2], data[3]]));

    let (drv8353_status1, drv8353_status2) = if fault_details
    {
        (Some(FaultDetailsByte45::from_bits_truncate(u16::from_le_bytes([data[4], data[5]]))),
        Some(FaultDetailsByte67::from_bits_truncate(u16::from_le_bytes([data[6], data[7]]))))
    } else {
        (None, None)
    };

    Some(FaultDetailsFrame {
    fault_code,
    fault_mask_code,
    drv8353_status1,
    drv8353_status2,
})

}

enum ParameterResponseFrameVal
{
    ParameterResponseByte47(ParameterResponseByte47),
    GroupedBitmap2(GroupedBitmap2),
    RawBytes([u8; 4]),
}
pub struct ParameterResponseFrame
{
    opcode_echo:ParameterResponseByte0,
    err_code: ParameterResponseByte1,
    parameter_id: ParameterResponseByte23,
    val: ParameterResponseFrameVal,
}

pub fn parameter_response_frame(data: &[u8]) -> Option<ParameterResponseFrame>
{
    if data.len() < 8 {
        return None;
    }

    let Some(opcode_echo) =
        ParameterResponseByte0::from_repr(data[0])
        else {return None};
    let Some(err_code) =
        ParameterResponseByte1::from_repr(data[1])
        else {return None};
    let parameter_id: GeneralParametersByte23 =
        ParameterResponseByte23::from_repr(u16::from_le_bytes(
            [data[2], data[3]]
            )
        ).unwrap();

    let val = match err_code
    {
        ParameterResponseByte1::SUCCESS => 
            ParameterResponseFrameVal::ParameterResponseByte47(
                ParameterResponseByte47::from_repr(
                    u32::from_le_bytes(
                        [data[4], data[5], data[6], data[7]]
                    )
                ).unwrap(),
            ),

        ParameterResponseByte1::APPLYREADBACKMISMATCH =>
        ParameterResponseFrameVal::GroupedBitmap2(
            GroupedBitmap2::from_bits_truncate(
                u32::from_le_bytes(
                    [data[4], data[5], data[6], data[7]
                    ]
                )
            )
        ),
        _ => ParameterResponseFrameVal::RawBytes([data[4],data[5],data[6],data[7]]),
        
    };
    

    Some(ParameterResponseFrame {
        opcode_echo,
        err_code,
        parameter_id,
        val,
    })
}