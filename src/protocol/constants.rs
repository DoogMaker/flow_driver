use bitflags::bitflags;


pub const TARGETSPEED: i16 = 10;
pub const OPENLOOPVOLTAGE: i16 = 100;
pub const TARGETTORQUE: i16 = 100;
pub const TARGETSTATE: i32 = 10000;

pub enum AllFrameBasic
{
    Command_Frame               = 0x100,
    Status_Feedback_Frame       = 0x200,
    Fault_Details_Frame         = 0x300,
    Parameter_Response_Frame    = 0x400,
}

pub enum EncodeCommand
{
    Target_Speed        = 0x01,
    Openloop_Voltage    = 0x02,
    Switch_Models       = 0x03,
    Set_Direction       = 0x04,
    Target_Torque       = 0x05,
    Target_State        = 0x06,
    Keep_Alive          = 0x07,
    Ask_Status_Feedback = 0x08,
    Set_Node_Number     = 0x09,
    General_Parameters  = 0x0A,
}

pub enum SwitchModelsByte1
{
    IDLE        = 0x00,
    Openloop    = 0x01,
    Align       = 0x02,
    Torque      = 0x03,
    Speed       = 0x04,
    Position    = 0x05,
    Fault       = 0x06,
}

pub enum SetDirectionByte1
{
    Forward_Rotation    = 0x00,
    Backward_Rotation   = 0x01,
}

pub enum SetNodeNumberbyte1
{
    Set_Node_Number1     = 1,
    Set_Node_Number2     = 2,
    Set_Node_Number3     = 3,
    Set_Node_Number4     = 4,
    Set_Node_Number5     = 5,
    Set_Node_Number6     = 6,
    Set_Node_Number7     = 7,
    Set_Node_Number8     = 8,
    Set_Node_Number9     = 9,
    Set_Node_Number10    = 10,
    Set_Node_Number11    = 11,
    Set_Node_Number12    = 12,
}

pub enum GeneralParametersByte1
{
    Read_Effectiveval   = 0x00,
    Read_Temporaryval   = 0x01,
    Write_Temporaryval  = 0x02,
    Submit_Effectiveval = 0x03,
    Save_Effectiveval   = 0x04,
    Reset_Default       = 0x05,
}

pub enum GeneralParametersByte23
{
    //0x01
    Speed_Loop_Kp       = 0x0100,
    Speed_Loop_Ki       = 0x0101,
    Speed_Loop_Kd       = 0x0102,
    // F32 0-1000 0-10000 0-1000


    ID_Loop_Kp          = 0x0103,
    ID_Loop_Ki          = 0x0104,
    ID_Loop_Kd          = 0x0105,
    // F32 0-1000 0-10000 0-1000


    IP_Loop_Kp_Limit    = 0x0106,
    IP_Loop_Ki_Limit    = 0x0107,
    IP_Loop_Kd_Limit    = 0x0108,
    // F32 0-1000 0-10000 0-1000


    Pose_loop_Kp        = 0x0109,
    Pose_loop_Ki        = 0x010A,
    Pose_loop_Kd        = 0x010B,
    // F32 0-1000 0-10000 0-1000

    Feedforward_Vel_Gain    = 0x010C,
    Feedforward_Inertia     = 0x010D,
    Feedforward_Viscosity   = 0x010E,
    Feedforward_Pos_Tc      = 0x010F,
    Feedforward_Acc_TC      = 0x0110,
    // F32 0-2 0-100 0-100 0.0005-10 0.0002-10

    Node_ID = 0x111,
    // U32 1-12

    Alignment_Voltage = 0x112,
    Single_Step_Delay = 0x113,
    Averaging_Windows = 0x114,
    // U32 0-12000 mV 0-60000 ms 0-60000 ms

    Position_Error_Timeouttick  = 0x115,
    Soft_Limit_Timeouttick      = 0x116,
    Position_Error_Threshold    = 0x117,
    // U32 1~60000 U32 1~60000 F32 0.0001~6.283

    Soft_Limit_Lower    = 0x118,
    Soft_Limit_Upper    = 0x119,
    // F32 -π~π

    Connection_loss_Running = 0x11A,
    Connection_loss_Standby = 0x11B,
    Control_Authority_Window = 0x11C,
    // U32 1~2999 ms 1~3000 ms 0~5000 ms

    Velocity_Ramp = 0x011D,
    // F32 0.001~10000 rad/s^2

    VF_Gain                                 = 0x011E,
    Voltage_compensation                    = 0x011F, 
    Encoder_Angle                           = 0x0120,
    Encoder_Velocity_Filter_Coefficients    = 0x0121,
    Output_Angle                            = 0x0122,
    Output_Velcotiyfilter_Coefficients      = 0x0123,
    // F32 0~1 0~5 0.001~1 0.001~1 0.001~1 0.001~1

    
    //0x02
    //最小端
    Page_Invalid_Mask_A      = 0x0200,
    Page_Invalid_Mask_B      = 0x0201,

    Load_Ok                  = 0x0202,
    Configured               = 0x0203,
    Save_Ok                  = 0x0204,

    Apply_Mismatch_Count     = 0x0205,
    Apply_Fail_Mask          = 0x0206,
    Stage_Pending            = 0x0207,
    Dirty_Field_Count        = 0x0208,

    Active_Node_Id           = 0x0209,
    Drive_Mode               = 0x020A,
    Save_Allowed             = 0x020B,

    Image_Version            = 0x020C,
    Write_Counter            = 0x020D,
    Last_Apply_Result        = 0x020E,

    Initialized              = 0x020F,
    Motor_Ready              = 0x0210,
    Calibration_Save_Pending = 0x0211,

    Dirty_Group_Mask         = 0x0212,
    Can_Filter_Applied       = 0x0213,
    Diagnostics_Version      = 0x0214,


    //0x03
    Ia_Gain           = 0x0300,
    Ib_Gain           = 0x0301,
    Vbus_Gain         = 0x0302,

    Ntc_R25_Cal       = 0x0303,
    Kt_Measured       = 0x0304,

    Zero_Offset       = 0x0305,

    Profile_ID        = 0x0306,
    Power_Class       = 0x0307,

    Encoder_Offset    = 0x0308,
    Encoder_Direction = 0x0309,    
}

//=========================================================================================

pub enum FeedBackFrame
{
    Status_Feedback     = 0x200,
    Fault_Details       = 0x300,
    Parameter_Response  = 0x400,
}

pub enum StatusFeedbackByte0
{
    IDLE    = 0x00,
    Running = 0x01,
    Fault   = 0x02,
}

pub const SPEED: i16 = 10;
pub const PHASEACURRENT: i16 = 100; 
pub const BUSVOLTAGE: i16 = 10;

pub type StatusFeedbackByte5 = SwitchModelsByte1;


pub enum FaultDetailsByte0
{
    None            = 0x00,
    HARDWARE        = 0x01,
    STALL           = 0x02,
    ENCODER_STALL   = 0x03,
    OVERTEMP        = 0x04,
    OVERVOLT        = 0x05,
    UNDERVOLT       = 0x06,
    OVERSPEED       = 0x07,
    PHASE_OC        = 0x08,
    I2T             = 0x09,
    POS_ERROR       = 0x0A,
    SOFT_LIMIT      = 0x0B,
    COMM_TIMEOUT    = 0x0C,
}

pub enum FaultDetailsByte1
{
    IN_BACKOFF      = 0x01,
    VALID_STATUS    = 0x03,
}

bitflags! {
    pub struct FaultDetailsByte23: u16 
    {
        const HARDWARE        = 0x0002;
        const STALL           = 0x0004;
        const ENCODER_STALL   = 0x0008;
        const OVERTEMP        = 0x0010;
        const OVERVOLT        = 0x0020;
        const UNDERVOLT       = 0x0040;
        const OVERSPEED       = 0x0080;
        const PHASE_OC        = 0x0100;
        const I2T             = 0x0200;
        const POS_ERROR       = 0x0400;
        const SOFT_LIMIT      = 0x0800;
        const COMM_TIMEOUT    = 0x1000;
    }

    pub struct FaultDetailsByte45: u16
    {
        const NONE     = 0x0000;
        const VDS_LC   = 0x0001;
        const VDS_HC   = 0x0002;
        const VDS_LB   = 0x0004;
        const VDS_HB   = 0x0008;
        const VDS_LA   = 0x0010;
        const VDS_HA   = 0x0020;
        const OTSD     = 0x0040;
        const UVLO     = 0x0080;
        const GDF      = 0x0100;
        const VDS_OCP  = 0x0200;
        const FAULT    = 0x0400;
    }
    pub struct FaultDetailsByte67: u16
    {
        const NONE     = 0x0000;
        const VGS_LC   = 0x0001;
        const VGS_HC   = 0x0002;
        const VGS_LB   = 0x0004;
        const VGS_HB   = 0x0008;
        const VGS_LA   = 0x0010;
        const VGS_HA   = 0x0020;
        const GDUV     = 0x0040;
        const OTW      = 0x0080;
        const SC_OC    = 0x0100;
        const SB_OC    = 0x0200;
        const SA_OC    = 0x0400;
    }
}

//==============================================================================================
const CONST_PARAMETERRESPONSEBYTE0: u8 = 0x80;

pub enum ParameterResponseByte0
{
    Read_Effectiveval   = 0x00 | CONST_PARAMETERRESPONSEBYTE0,
    Read_Temporaryval   = 0x01 | CONST_PARAMETERRESPONSEBYTE0,
    Write_Temporaryval  = 0x02 | CONST_PARAMETERRESPONSEBYTE0,
    Write_Effectiveval  = 0x03 | CONST_PARAMETERRESPONSEBYTE0,
    Submit_Effectiveval = 0x04 | CONST_PARAMETERRESPONSEBYTE0,
    Save_Effectiveval   = 0x05 | CONST_PARAMETERRESPONSEBYTE0,
}


pub enum ParameterResponseByte1
{
    SUCCESS                 = 0x00,
    INVALID_PARAM           = 0x01,
    UNKNOWN_PARAM_ID        = 0x02,
    NO_PERMISSION           = 0x03,
    TYPE_UNAVAILABLE        = 0x04,
    VALUE_OUT_OF_RANGE      = 0x05,
    GROUP_CHECK_FAILED      = 0x06,
    PARAM_NOT_INITIALIZED   = 0x07,
    NOT_IDLE                = 0x08,
    PENDING_UNCOMMITTED     = 0x09,
    FLASH_SAVE_FAILED       = 0x0A,
    APPLY_READBACK_MISMATCH = 0x0B,
}

pub type ParameterResponseByte23 = GeneralParametersByte23;

pub enum ParameterResponseByte47
{
    Faild_NoWrite = 0x0000,
}

bitflags! {
    pub struct GroupedBitmap1: u32 {
        const NONE              = 0;
        const BLANK_PAGE        = 1 << 0;  // 空白页
        const MAGIC_MISMATCH    = 1 << 1;  // magic 不符
        const VERSION_MISMATCH  = 1 << 2;  // 结构版本不符
        const CHECKSUM_MISMATCH = 1 << 3;  // 校验和不符
        const CONTENT_INVALID   = 1 << 4;  // 内容校验不通过
    }

    pub struct GroupedBitmap2: u32 {
        const NONE                  = 0;
        const SPEED                 = 1 << 1;  // 0x0000_0002
        const ID                    = 1 << 2;  // 0x0000_0004
        const IQ                    = 1 << 3;  // 0x0000_0008
        const POS                   = 1 << 4;  // 0x0000_0010
        const FEEDFORWARD           = 1 << 5;  // 0x0000_0020
        const VELOCITY_RAMP         = 1 << 6;  // 0x0000_0040
        const NODE_ID               = 1 << 7;  // 0x0000_0080
        const ALIGNMENT             = 1 << 8;  // 0x0000_0100
        const POSITION_FOLLOW       = 1 << 9;  // 0x0000_0200
        const SOFT_LIMIT            = 1 << 10; // 0x0000_0400
        const COMM_LOSS             = 1 << 11; // 0x0000_0800
        const ZERO_OFFSET           = 1 << 12; // 0x0000_1000
        const CALIBRATION           = 1 << 13; // 0x0000_2000
        const VF_ENCODER_FILTER     = 1 << 14; // 0x0000_4000
    }
}