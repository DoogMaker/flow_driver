use super::constants::*;
use repr_cast::repr_cast;
use bitflags::bitflags;


struct StatusFeedbackFrame {
    system_state: StatusFeedbackByte0,
    rotation_speed: f32,
    current_a: f32,
    model: StatusFeedbackByte5,
    bus_voltage: f32,

}

fn status_feedback_frame(data: &[u8]) -> Option<StatusFeedbackFrame>
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

struct FaultDetailsFrame {
    fault_code: FaultDetailsByte0,
    fault_mask_code: FaultDetailsByte23,
    drv8353_status1: Option<FaultDetailsByte45>,
    drv8353_status2: Option<FaultDetailsByte67>,
}

fn fault_details_frame(data: &[u8]) -> Option<FaultDetailsFrame>
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


