use bitflags::bitflags;
use repr_cast::repr_cast;

pub const TARGETSPEED: f32 = 10.0;
pub const OPENLOOPVOLTAGE: f32 = 100.0;
pub const TARGETTORQUE: f32 = 100.0;
pub const TARGETSTATE: f32 = 10000.0;
pub const MAX_NODE_NUMBER: u8 = 12;
pub const CURRENT_A: f32 = 100.0;

#[repr(u16)]
pub enum AllFrameBasic {
    CommandFrame = 0x100,
    StatusFeedbackFrame = 0x200,
    FaultDetailsFrame = 0x300,
    ParameterResponseFrame = 0x400,
}

#[repr(u8)]
pub enum EncodeCommand {
    TargetSpeed = 0x01,
    OpenloopVoltage = 0x02,
    SwitchModels = 0x03,
    SetDirection = 0x04,
    TargetTorque = 0x05,
    TargetState = 0x06,
    KeepAlive = 0x07,
    AskStatusFeedback = 0x08,
    SetNodeNumber = 0x09,
    GeneralParameters = 0x0A,
}

#[repr_cast(u8)]
pub enum SwitchModelsByte1 {
    IDLE = 0x00,
    Openloop = 0x01,
    Align = 0x02,
    Torque = 0x03,
    Speed = 0x04,
    Position = 0x05,
    Fault = 0x06,
}

#[repr(u8)]
pub enum SetDirectionByte1 {
    ForwardRotation = 0x00,
    BackwardRotation = 0x01,
}


#[repr(u8)]
pub enum GeneralParametersByte1 {
    ReadEffectiveval = 0x00,
    ReadTemporaryval = 0x01,
    WriteTemporaryval = 0x02,
    SubmitEffectiveval = 0x03,
    SaveEffectiveval = 0x04,
    ResetDefault = 0x05,
}

#[repr(u16)]
pub enum GeneralParametersByte23 {
    None = 0x0000,
    //0x01
    SpeedLoopKp = 0x0100,
    SpeedLoopKi = 0x0101,
    SpeedLoopKd = 0x0102,
    // F32 0-1000 0-10000 0-1000
    IDLoopKp = 0x0103,
    IDLoopKi = 0x0104,
    IDLoopKd = 0x0105,
    // F32 0-1000 0-10000 0-1000
    IPLoopKpLimit = 0x0106,
    IPLoopKiLimit = 0x0107,
    IPLoopKdLimit = 0x0108,
    // F32 0-1000 0-10000 0-1000
    PoseloopKp = 0x0109,
    PoseloopKi = 0x010A,
    PoseloopKd = 0x010B,
    // F32 0-1000 0-10000 0-1000
    FeedforwardVelGain = 0x010C,
    FeedforwardInertia = 0x010D,
    FeedforwardViscosity = 0x010E,
    FeedforwardPosTc = 0x010F,
    FeedforwardAccTC = 0x0110,
    // F32 0-2 0-100 0-100 0.0005-10 0.0002-10
    NodeID = 0x111,
    // U32 1-12
    AlignmentVoltage = 0x112,
    SingleStepDelay = 0x113,
    AveragingWindows = 0x114,
    // U32 0-12000 mV 0-60000 ms 0-60000 ms
    PositionErrorTimeouttick = 0x115,
    SoftLimitTimeouttick = 0x116,
    PositionErrorThreshold = 0x117,
    // U32 1~60000 U32 1~60000 F32 0.0001~6.283
    SoftLimitLower = 0x118,
    SoftLimitUpper = 0x119,
    // F32 -π~π
    ConnectionlossRunning = 0x11A,
    ConnectionlossStandby = 0x11B,
    ControlAuthorityWindow = 0x11C,
    // U32 1~2999 ms 1~3000 ms 0~5000 ms
    VelocityRamp = 0x011D,
    // F32 0.001~10000 rad/s^2
    VFGain = 0x011E,
    Voltagecompensation = 0x011F,
    EncoderAngle = 0x0120,
    EncoderVelocityFilterCoefficients = 0x0121,
    OutputAngle = 0x0122,
    OutputVelcotiyfilterCoefficients = 0x0123,
    // F32 0~1 0~5 0.001~1 0.001~1 0.001~1 0.001~1

    //0x02
    //最小端
    PageInvalidMaskA = 0x0200,
    PageInvalidMaskB = 0x0201,

    LoadOk = 0x0202,
    Configured = 0x0203,
    SaveOk = 0x0204,

    ApplyMismatchCount = 0x0205,
    ApplyFailMask = 0x0206,
    StagePending = 0x0207,
    DirtyFieldCount = 0x0208,

    ActiveNodeId = 0x0209,
    DriveMode = 0x020A,
    SaveAllowed = 0x020B,

    ImageVersion = 0x020C,
    WriteCounter = 0x020D,
    LastApplyResult = 0x020E,

    Initialized = 0x020F,
    MotorReady = 0x0210,
    CalibrationSavePending = 0x0211,

    DirtyGroupMask = 0x0212,
    CanFilterApplied = 0x0213,
    DiagnosticsVersion = 0x0214,

    //0x03
    IaGain = 0x0300,
    IbGain = 0x0301,
    VbusGain = 0x0302,

    NtcR25Cal = 0x0303,
    KtMeasured = 0x0304,

    ZeroOffset = 0x0305,

    ProfileID = 0x0306,
    PowerClass = 0x0307,

    EncoderOffset = 0x0308,
    EncoderDirection = 0x0309,
}

//=========================================================================================

#[repr(u16)]
pub enum FeedBackFrame {
    StatusFeedback = 0x200,
    FaultDetails = 0x300,
    ParameterResponse = 0x400,
}

#[repr_cast(u8)]
pub enum StatusFeedbackByte0 {
    IDLE = 0x00,
    Running = 0x01,
    Fault = 0x02,
}

pub const SPEED: f32 = 10.0;
pub const PHASEACURRENT: f32 = 100.0;
pub const BUSVOLTAGE: f32 = 10.0;

pub type StatusFeedbackByte5 = SwitchModelsByte1;

#[repr_cast(u8)]
pub enum FaultDetailsByte0 {
    None = 0x00,
    HARDWARE = 0x01,
    STALL = 0x02,
    ENCODERSTALL = 0x03,
    OVERTEMP = 0x04,
    OVERVOLT = 0x05,
    UNDERVOLT = 0x06,
    OVERSPEED = 0x07,
    PHASEOC = 0x08,
    I2T = 0x09,
    POSERROR = 0x0A,
    SOFTLIMIT = 0x0B,
    COMMTIMEOUT = 0x0C,
}

#[repr_cast(u8)]
pub enum FaultDetailsByte1 {
    INBACKOFF = 0x01,
    VALIDSTATUS = 0x03,
}

bitflags! {
    pub struct FaultDetailsByte23: u16
    {
        const HARDWARE        = 0x0002;
        const STALL           = 0x0004;
        const ENCODERSTALL   = 0x0008;
        const OVERTEMP        = 0x0010;
        const OVERVOLT        = 0x0020;
        const UNDERVOLT       = 0x0040;
        const OVERSPEED       = 0x0080;
        const PHASEOC        = 0x0100;
        const I2T             = 0x0200;
        const POSERROR       = 0x0400;
        const SOFTLIMIT      = 0x0800;
        const COMMTIMEOUT    = 0x1000;
    }

    pub struct FaultDetailsByte45: u16
    {
        const NONE     = 0x0000;
        const VDSLC   = 0x0001;
        const VDSHC   = 0x0002;
        const VDSLB   = 0x0004;
        const VDSHB   = 0x0008;
        const VDSLA   = 0x0010;
        const VDSHA   = 0x0020;
        const OTSD     = 0x0040;
        const UVLO     = 0x0080;
        const GDF      = 0x0100;
        const VDSOCP  = 0x0200;
        const FAULT    = 0x0400;
    }
    pub struct FaultDetailsByte67: u16
    {
        const NONE     = 0x0000;
        const VGSLC   = 0x0001;
        const VGSHC   = 0x0002;
        const VGSLB   = 0x0004;
        const VGSHB   = 0x0008;
        const VGSLA   = 0x0010;
        const VGSHA   = 0x0020;
        const GDUV     = 0x0040;
        const OTW      = 0x0080;
        const SCOC    = 0x0100;
        const SBOC    = 0x0200;
        const SAOC    = 0x0400;
    }
}

//==============================================================================================
pub const CONST_PARAMETERRESPONSEBYTE0: u8 = 0x80;

#[repr(u8)]
pub enum ParameterResponseByte0 {
    ReadEffectiveval = 0x00 | CONST_PARAMETERRESPONSEBYTE0,
    ReadTemporaryval = 0x01 | CONST_PARAMETERRESPONSEBYTE0,
    WriteTemporaryval = 0x02 | CONST_PARAMETERRESPONSEBYTE0,
    WriteEffectiveval = 0x03 | CONST_PARAMETERRESPONSEBYTE0,
    SubmitEffectiveval = 0x04 | CONST_PARAMETERRESPONSEBYTE0,
    SaveEffectiveval = 0x05 | CONST_PARAMETERRESPONSEBYTE0,
}

#[repr(u8)]
pub enum ParameterResponseByte1 {
    SUCCESS = 0x00,
    INVALIDPARAM = 0x01,
    UNKNOWNPARAMID = 0x02,
    NOPERMISSION = 0x03,
    TYPEUNAVAILABLE = 0x04,
    VALUEOUTOFRANGE = 0x05,
    GROUPCHECKFAILED = 0x06,
    PARAMNOTINITIALIZED = 0x07,
    NOTIDLE = 0x08,
    PENDINGUNCOMMITTED = 0x09,
    FLASHSAVEFAILED = 0x0A,
    APPLYREADBACKMISMATCH = 0x0B,
}

pub type ParameterResponseByte23 = GeneralParametersByte23;

#[repr(u32)]
pub enum ParameterResponseByte47 {
    FaildNoWrite = 0x0000,
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
