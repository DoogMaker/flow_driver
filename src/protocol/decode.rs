use super::constants::*;
use repr_cast::repr_cast;
use bitflags::bitflags;


#[derive(Debug)]
pub struct StatusFeedbackFrame
{
    system_state: StatusFeedbackByte0,
    rotation_speed: f32,
    current_a: f32,
    model: StatusFeedbackByte5,
    bus_voltage: f32,

}

pub fn unknow_decode(data: &[u8]) -> Option<[u8; 8]>
{
    if data.len() < 8 {
        println!("data length must be 8");
        return None;
    }

    let mut raw_bytes = [0u8; 8];
    raw_bytes.copy_from_slice(&data[0..8]);
    Some(raw_bytes)
}

pub fn status_feedback_frame(data: &[u8]) -> Option<StatusFeedbackFrame>
{
    if data.len() < 8 {
        println!("data length must be 8");
        return None;
    }

    let Some(system_state) = StatusFeedbackByte0::from_repr(data[0])
        else {
            panic!("system_state is invalid")
        };

    let rotation_speed = i16::from_le_bytes([data[1], data[2]]) as f32 / TARGETSPEED;

    let current_a = i16::from_le_bytes([data[3], data[4]]) as f32 / CURRENT_A;

    let Some(model) = StatusFeedbackByte5::from_repr(data[5])
        else {
            println!("model is invalid");
            return None;
        };

    let bus_voltage = i16::from_le_bytes([data[6], data[7]]) as f32 / BUSVOLTAGE;

    Some(StatusFeedbackFrame {
        system_state,
        rotation_speed,
        current_a,
        model,
        bus_voltage,
    })
}

#[derive(Debug)]
pub struct FaultDetailsFrame
{
    pub fault_code: FaultDetailsByte0,
    pub fault_mask_code: FaultDetailsByte23,
    pub drv8353_status1: Option<FaultDetailsByte45>,
    pub drv8353_status2: Option<FaultDetailsByte67>,
}

pub fn fault_details_frame(data: &[u8]) -> Option<FaultDetailsFrame>
{
    if data.len() < 8 {
        println!("data length must be 8");
        return None;
    }

    let Some(fault_code) = FaultDetailsByte0::from_repr(data[0])
        else {
            println!("fault_code is invalid");
            return None;
        };

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

#[derive(Debug)]
enum ParameterResponseFrameVal
{
    ParameterResponseByte47(ParameterResponseByte47),
    GroupedBitmap2(GroupedBitmap2),
    RawBytes([u8; 4]),
    val(f32),
}

#[derive(Debug)]
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
        println!("data length must be 8");
        return None;
    }

    let Some(opcode_echo) =
        ParameterResponseByte0::from_repr(data[0])
        else {
            println!("opcode_echo is invalid");
            return None;
        };

    let Some(err_code) =
        ParameterResponseByte1::from_repr(data[1])
        else {
            panic!("err_code is invalid")
        };
    let parameter_id: GeneralParametersByte23 =
        ParameterResponseByte23::from_repr(u16::from_le_bytes(
            [data[2], data[3]]
            )
        ).unwrap();

    let val = match opcode_echo
    {
        ParameterResponseByte0::ReadEffectiveval | ParameterResponseByte0::ReadTemporaryval =>
            match err_code
            {
                ParameterResponseByte1::SUCCESS =>
                    ParameterResponseFrameVal::val(
                        f32::from_le_bytes(
                            [data[4], data[5], data[6], data[7]]
                        )
                    ),
                ParameterResponseByte1::APPLYREADBACKMISMATCH =>
                    ParameterResponseFrameVal::ParameterResponseByte47(
                        ParameterResponseByte47::FaildNoWrite
                    ),
                _ => ParameterResponseFrameVal::RawBytes([data[4],data[5],data[6],data[7]]),
            }

        ParameterResponseByte0::SubmitEffectiveval =>
            match err_code
            {
                ParameterResponseByte1::APPLYREADBACKMISMATCH =>
                    ParameterResponseFrameVal::GroupedBitmap2(
                        GroupedBitmap2::from_bits_truncate(
                            u32::from_le_bytes(
                                [data[4], data[5], data[6], data[7]]
                            )
                        )
                    ),
                _ => ParameterResponseFrameVal::RawBytes([data[4],data[5],data[6],data[7]]),
            },

        _ => ParameterResponseFrameVal::RawBytes([data[4],data[5],data[6],data[7]]),


            
    };

    Some(ParameterResponseFrame {
        opcode_echo,
        err_code,
        parameter_id,
        val,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_feedback_frame() {
        let f = status_feedback_frame(&[0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0, 0x00]).unwrap();
        let state = f.system_state as u8;
        let model = f.model as u8;
        println!("idle: state={} speed={} current={} model={} bus={}", state, f.rotation_speed, f.current_a, model, f.bus_voltage);
        assert_eq!(state, 0);
        assert_eq!(f.rotation_speed, 0.0);
        assert_eq!(f.current_a, 0.0);
        assert_eq!(model, 0);
        assert_eq!(f.bus_voltage, 24.0);

        let f = status_feedback_frame(&[0x01, 0x7D, 0x00, 0x7D, 0x00, 0x01, 0xF0, 0x00]).unwrap();
        let state = f.system_state as u8;
        let model = f.model as u8;
        println!("openloop: state={} speed={} current={} model={} bus={}", state, f.rotation_speed, f.current_a, model, f.bus_voltage);
        assert_eq!(state, 1);
        assert_eq!(f.rotation_speed, 12.5);
        assert_eq!(f.current_a, 1.25);
        assert_eq!(model, 1);
        assert_eq!(f.bus_voltage, 24.0);

        let f = status_feedback_frame(&[0x02, 0x00, 0x00, 0x00, 0x00, 0x06, 0xF0, 0x00]).unwrap();
        let state = f.system_state as u8;
        let model = f.model as u8;
        println!("fault: state={} speed={} current={} model={} bus={}", state, f.rotation_speed, f.current_a, model, f.bus_voltage);
        assert_eq!(state, 2);
        assert_eq!(model, 6);
        assert_eq!(f.bus_voltage, 24.0);

        let f = status_feedback_frame(&[0x01, 0xE0, 0xFF, 0xCE, 0xFF, 0x01, 0xF0, 0x00]).unwrap();
        println!("negative: speed={} current={}", f.rotation_speed, f.current_a);
        assert!((f.rotation_speed + 3.2).abs() < 0.001);
        assert!((f.current_a + 0.5).abs() < 0.001);

        let f = status_feedback_frame(&[0x01, 0x00, 0x00, 0x00, 0x00, 0x04, 0xF0, 0x00]).unwrap();
        let model = f.model as u8;
        println!("speed mode: model={}", model);
        assert_eq!(model, 4);
    }

    #[test]
    fn test_fault_details_frame() {
        let f = fault_details_frame(&[0x08, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let code = f.fault_code as u8;
        println!("phase_oc: code={} mask={:04X} status1={} status2={}", code, f.fault_mask_code.bits(), f.drv8353_status1.is_some(), f.drv8353_status2.is_some());
        assert_eq!(code, 8);
        assert_eq!(f.fault_mask_code.bits(), 0x0100);
        assert!(f.drv8353_status1.is_none());
        assert!(f.drv8353_status2.is_none());

        let f = fault_details_frame(&[0x01, 0x03, 0x02, 0x00, 0x00, 0x02, 0x00, 0x00]).unwrap();
        let code = f.fault_code as u8;
        println!("vds_ocp: code={} mask={:04X} status1={} status2={}", code, f.fault_mask_code.bits(), f.drv8353_status1.is_some(), f.drv8353_status2.is_some());
        assert_eq!(code, 1);
        assert_eq!(f.fault_mask_code.bits(), 0x0002);
        assert!(f.drv8353_status1.is_some());
        assert!(f.drv8353_status2.is_some());

        let f = fault_details_frame(&[0x0C, 0x01, 0x00, 0x10, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let code = f.fault_code as u8;
        println!("comm_timeout: code={} mask={:04X}", code, f.fault_mask_code.bits());
        assert_eq!(code, 12);
        assert_eq!(f.fault_mask_code.bits(), 0x1000);

        let f = fault_details_frame(&[0x05, 0x01, 0x20, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let code = f.fault_code as u8;
        println!("overvolt: code={} mask={:04X}", code, f.fault_mask_code.bits());
        assert_eq!(code, 5);
        assert_eq!(f.fault_mask_code.bits(), 0x0020);

        let f = fault_details_frame(&[0x09, 0x01, 0x06, 0x02, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let code = f.fault_code as u8;
        println!("i2t_stall: code={} mask={:04X}", code, f.fault_mask_code.bits());
        assert_eq!(code, 9);
        assert_eq!(f.fault_mask_code.bits(), 0x0206);
    }

    #[test]
    fn test_parameter_response_frame() {
        let f = parameter_response_frame(&[0x82, 0x05, 0x00, 0x01, 0x00, 0x00, 0xFA, 0x45]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        let id = f.parameter_id as u16;
        println!("write range: op={:02X} err={} id={:04X}", op, err, id);
        assert_eq!(op, 0x82);
        assert_eq!(err, 5);
        assert_eq!(id, 0x0100);

        let f = parameter_response_frame(&[0x82, 0x03, 0x05, 0x03, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        let id = f.parameter_id as u16;
        println!("write access: op={:02X} err={} id={:04X}", op, err, id);
        assert_eq!(op, 0x82);
        assert_eq!(err, 3);
        assert_eq!(id, 0x0305);

        let f = parameter_response_frame(&[0x82, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        let id = f.parameter_id as u16;
        println!("write arg: op={:02X} err={} id={:04X}", op, err, id);
        assert_eq!(op, 0x82);
        assert_eq!(err, 1);
        assert_eq!(id, 0x0100);

        let f = parameter_response_frame(&[0x83, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        let id = f.parameter_id as u16;
        println!("commit ok: op={:02X} err={} id={:04X}", op, err, id);
        assert_eq!(op, 0x83);
        assert_eq!(err, 0);
        assert_eq!(id, 0x0000);

        let f = parameter_response_frame(&[0x83, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let err = f.err_code as u8;
        println!("commit pending: err={}", err);
        assert_eq!(err, 9);

        let f = parameter_response_frame(&[0x83, 0x0A, 0x00, 0x00, 0x02, 0x00, 0x00, 0x00]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        println!("commit apply mask: op={:02X} err={}", op, err);
        assert_eq!(op, 0x83);
        assert_eq!(err, 10);

        let f = parameter_response_frame(&[0x84, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        println!("save ok: op={:02X} err={}", op, err);
        assert_eq!(op, 0x84);
        assert_eq!(err, 0);

        let f = parameter_response_frame(&[0x84, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let err = f.err_code as u8;
        println!("save not idle: err={}", err);
        assert_eq!(err, 8);

        let f = parameter_response_frame(&[0x85, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        println!("reset ok: op={:02X} err={}", op, err);
        assert_eq!(op, 0x85);
        assert_eq!(err, 0);

        let f = parameter_response_frame(&[0x80, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        println!("get arg: op={:02X} err={}", op, err);
        assert_eq!(op, 0x80);
        assert_eq!(err, 1);

        // (1) 读 active：kp_speed = 0.10f，[4:7] = CD CC CC 3D
        let f = parameter_response_frame(&[0x80, 0x00, 0x00, 0x01, 0xCD, 0xCC, 0xCC, 0x3D]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        let id = f.parameter_id as u16;
        println!("get active kp_speed: op={:02X} err={} id={:04X} value=0.10f", op, err, id);
        assert_eq!(op, 0x80);
        assert_eq!(err, 0);
        assert_eq!(id, 0x0100);

        // (2) 读 stage：同一字段，byte0 = 0x01|0x80
        let f = parameter_response_frame(&[0x81, 0x00, 0x00, 0x01, 0xCD, 0xCC, 0xCC, 0x3D]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        let id = f.parameter_id as u16;
        println!("get stage kp_speed: op={:02X} err={} id={:04X} value=0.10f", op, err, id);
        assert_eq!(op, 0x81);
        assert_eq!(err, 0);
        assert_eq!(id, 0x0100);

        // (3) 读 U32：align_voltage_mv = 0x0112，值 600 = 0x00000258
        let f = parameter_response_frame(&[0x80, 0x00, 0x12, 0x01, 0x58, 0x02, 0x00, 0x00]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        let id = f.parameter_id as u16;
        println!("get active align_voltage_mv: op={:02X} err={} id={:04X} value=600", op, err, id);
        assert_eq!(op, 0x80);
        assert_eq!(err, 0);
        assert_eq!(id, 0x0112);

        // (4) 读诊断：0x020E last_apply_result = 0xFFFFFFFF
        let f = parameter_response_frame(&[0x80, 0x00, 0x0E, 0x02, 0xFF, 0xFF, 0xFF, 0xFF]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        let id = f.parameter_id as u16;
        println!("get active last_apply_result: op={:02X} err={} id={:04X} value=0xFFFFFFFF", op, err, id);
        assert_eq!(op, 0x80);
        assert_eq!(err, 0);
        assert_eq!(id, 0x020E);

        // (5) 写暂存成功：kp_speed = 0.2f，回显 CD CC 4C 3E
        let f = parameter_response_frame(&[0x82, 0x00, 0x00, 0x01, 0xCD, 0xCC, 0x4C, 0x3E]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        let id = f.parameter_id as u16;
        println!("write kp_speed: op={:02X} err={} id={:04X} echo=0.2f", op, err, id);
        assert_eq!(op, 0x82);
        assert_eq!(err, 0);
        assert_eq!(id, 0x0100);

        // 3.2 ② 写 ki_speed = 0.5f
        let f = parameter_response_frame(&[0x82, 0x00, 0x01, 0x01, 0x00, 0x00, 0x00, 0x3F]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        let id = f.parameter_id as u16;
        println!("write ki_speed: op={:02X} err={} id={:04X} echo=0.5f", op, err, id);
        assert_eq!(op, 0x82);
        assert_eq!(err, 0);
        assert_eq!(id, 0x0101);

        // (8) 写诊断项 0x020E：ACCESS
        let f = parameter_response_frame(&[0x82, 0x03, 0x0E, 0x02, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        let id = f.parameter_id as u16;
        println!("write diagnostic: op={:02X} err={} id={:04X}", op, err, id);
        assert_eq!(op, 0x82);
        assert_eq!(err, 3);
        assert_eq!(id, 0x020E);

        // (13) Commit 时参数 ID 非 0：ARG
        let f = parameter_response_frame(&[0x83, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        let id = f.parameter_id as u16;
        println!("commit id not zero: op={:02X} err={} id={:04X}", op, err, id);
        assert_eq!(op, 0x83);
        assert_eq!(err, 1);
        assert_eq!(id, 0x0100);

        // (16) Save 失败：PENDING / ACCESS / FLASH
        let f = parameter_response_frame(&[0x84, 0x09, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let err = f.err_code as u8;
        println!("save pending: err={}", err);
        assert_eq!(err, 9);

        let f = parameter_response_frame(&[0x84, 0x03, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let err = f.err_code as u8;
        println!("save access: err={}", err);
        assert_eq!(err, 3);

        let f = parameter_response_frame(&[0x84, 0x0A, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let err = f.err_code as u8;
        println!("save flash: err={}", err);
        assert_eq!(err, 10);

        // (17) 复位：非 IDLE 返回 NOT_IDLE，参数 ID 非 0 返回 ARG
        let f = parameter_response_frame(&[0x85, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let err = f.err_code as u8;
        println!("reset not idle: err={}", err);
        assert_eq!(err, 8);

        let f = parameter_response_frame(&[0x85, 0x01, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00]).unwrap();
        let op = f.opcode_echo as u8;
        let err = f.err_code as u8;
        let id = f.parameter_id as u16;
        println!("reset id not zero: op={:02X} err={} id={:04X}", op, err, id);
        assert_eq!(op, 0x85);
        assert_eq!(err, 1);
        assert_eq!(id, 0x0100);

        let f = unknow_decode(&[0x0A,0x06,0x00,0x00,0x00,0x00,0x00,0x00]).unwrap();
        assert!(f == [0x0A,0x06,0x00,0x00,0x00,0x00,0x00,0x00]);


    }
}