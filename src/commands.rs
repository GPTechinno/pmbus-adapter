/// All standard PMBus 1.4 command codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CommandCode {
    // General
    Page = 0x00,
    Operation = 0x01,
    OnOffConfig = 0x02,
    ClearFaults = 0x03,
    Phase = 0x04,
    PagePlusWrite = 0x05,
    PagePlusRead = 0x06,
    ZoneConfig = 0x07,
    ZoneActive = 0x08,

    // Store / Restore
    WriteProtect = 0x10,
    StoreDefaultAll = 0x11,
    RestoreDefaultAll = 0x12,
    StoreDefaultCode = 0x13,
    RestoreDefaultCode = 0x14,
    StoreUserAll = 0x15,
    RestoreUserAll = 0x16,
    StoreUserCode = 0x17,
    RestoreUserCode = 0x18,
    Capability = 0x19,
    Query = 0x1A,
    SmbalertMask = 0x1B,

    // Output voltage
    VoutMode = 0x20,
    VoutCommand = 0x21,
    VoutTrim = 0x22,
    VoutCalOffset = 0x23,
    VoutMax = 0x24,
    VoutMarginHigh = 0x25,
    VoutMarginLow = 0x26,
    VoutTransitionRate = 0x27,
    VoutDroop = 0x28,
    VoutScaleLoop = 0x29,
    VoutScaleMonitor = 0x2A,
    VoutMin = 0x2B,

    // Coefficients & power
    Coefficients = 0x30,
    PoutMax = 0x31,
    MaxDuty = 0x32,
    FrequencySwitch = 0x33,
    PowerMode = 0x34,
    VinOn = 0x35,
    VinOff = 0x36,
    Interleave = 0x37,
    IoutCalGain = 0x38,
    IoutCalOffset = 0x39,

    // Fan config/command
    FanConfig12 = 0x3A,
    FanCommand1 = 0x3B,
    FanCommand2 = 0x3C,
    FanConfig34 = 0x3D,
    FanCommand3 = 0x3E,
    FanCommand4 = 0x3F,

    // Fault/warn limits and responses — VOUT
    VoutOvFaultLimit = 0x40,
    VoutOvFaultResponse = 0x41,
    VoutOvWarnLimit = 0x42,
    VoutUvWarnLimit = 0x43,
    VoutUvFaultLimit = 0x44,
    VoutUvFaultResponse = 0x45,

    // IOUT
    IoutOcFaultLimit = 0x46,
    IoutOcFaultResponse = 0x47,
    IoutOcLvFaultLimit = 0x48,
    IoutOcLvFaultResponse = 0x49,
    IoutOcWarnLimit = 0x4A,
    IoutUcFaultLimit = 0x4B,
    IoutUcFaultResponse = 0x4C,

    // Over-temperature
    OtFaultLimit = 0x4F,
    OtFaultResponse = 0x50,
    OtWarnLimit = 0x51,

    // Under-temperature
    UtWarnLimit = 0x52,
    UtFaultLimit = 0x53,
    UtFaultResponse = 0x54,

    // VIN
    VinOvFaultLimit = 0x55,
    VinOvFaultResponse = 0x56,
    VinOvWarnLimit = 0x57,
    VinUvWarnLimit = 0x58,
    VinUvFaultLimit = 0x59,
    VinUvFaultResponse = 0x5A,

    // IIN
    IinOcFaultLimit = 0x5B,
    IinOcFaultResponse = 0x5C,
    IinOcWarnLimit = 0x5D,

    // Power good
    PowerGoodOn = 0x5E,
    PowerGoodOff = 0x5F,

    // Timing
    TonDelay = 0x60,
    TonRise = 0x61,
    TonMaxFaultLimit = 0x62,
    TonMaxFaultResponse = 0x63,
    ToffDelay = 0x64,
    ToffFall = 0x65,
    ToffMaxWarnLimit = 0x66,

    // POUT / PIN
    PoutOpFaultLimit = 0x68,
    PoutOpFaultResponse = 0x69,
    PoutOpWarnLimit = 0x6A,
    PinOpWarnLimit = 0x6B,

    // Status
    StatusByte = 0x78,
    StatusWord = 0x79,
    StatusVout = 0x7A,
    StatusIout = 0x7B,
    StatusInput = 0x7C,
    StatusTemperature = 0x7D,
    StatusCml = 0x7E,
    StatusOther = 0x7F,
    StatusMfrSpecific = 0x80,
    StatusFans12 = 0x81,
    StatusFans34 = 0x82,

    // Energy / KWH
    ReadKwhIn = 0x83,
    ReadKwhOut = 0x84,
    ReadKwhConfig = 0x85,

    // Telemetry — block reads
    ReadEin = 0x86,
    ReadEout = 0x87,

    // Telemetry — word reads
    ReadVin = 0x88,
    ReadIin = 0x89,
    ReadVcap = 0x8A,
    ReadVout = 0x8B,
    ReadIout = 0x8C,
    ReadTemperature1 = 0x8D,
    ReadTemperature2 = 0x8E,
    ReadTemperature3 = 0x8F,
    ReadFanSpeed1 = 0x90,
    ReadFanSpeed2 = 0x91,
    ReadFanSpeed3 = 0x92,
    ReadFanSpeed4 = 0x93,
    ReadDutyCycle = 0x94,
    ReadFrequency = 0x95,
    ReadPout = 0x96,
    ReadPin = 0x97,

    // Identification
    PmbusRevision = 0x98,
    MfrId = 0x99,
    MfrModel = 0x9A,
    MfrRevision = 0x9B,
    MfrLocation = 0x9C,
    MfrDate = 0x9D,
    MfrSerial = 0x9E,
    AppProfileSupport = 0x9F,

    // MFR telemetry limits
    MfrVinMin = 0xA0,
    MfrVinMax = 0xA1,
    MfrIinMax = 0xA2,
    MfrPinMax = 0xA3,
    MfrVoutMin = 0xA4,
    MfrVoutMax = 0xA5,
    MfrIoutMax = 0xA6,
    MfrPoutMax = 0xA7,
    MfrTambientMax = 0xA8,
    MfrTambientMin = 0xA9,
    MfrEfficiencyLl = 0xAA,
    MfrEfficiencyHl = 0xAB,
    MfrPinAccuracy = 0xAC,
    IcDeviceId = 0xAD,
    IcDeviceRev = 0xAE,

    // User data
    UserData00 = 0xB0,
    UserData01 = 0xB1,
    UserData02 = 0xB2,
    UserData03 = 0xB3,
    UserData04 = 0xB4,
    UserData05 = 0xB5,
    UserData06 = 0xB6,
    UserData07 = 0xB7,
    UserData08 = 0xB8,
    UserData09 = 0xB9,
    UserData10 = 0xBA,
    UserData11 = 0xBB,
    UserData12 = 0xBC,
    UserData13 = 0xBD,
    UserData14 = 0xBE,
    UserData15 = 0xBF,

    // MFR max temps
    MfrMaxTemp1 = 0xC0,
    MfrMaxTemp2 = 0xC1,
    MfrMaxTemp3 = 0xC2,

    // Extended command
    MfrSpecificCommandExt = 0xFE,
    PmbusCommandExt = 0xFF,
}

impl CommandCode {
    /// Return the raw u8 command code.
    pub fn code(self) -> u8 {
        self as u8
    }
}

impl From<CommandCode> for u8 {
    fn from(cmd: CommandCode) -> u8 {
        cmd as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_code_values() {
        assert_eq!(CommandCode::Page.code(), 0x00);
        assert_eq!(CommandCode::ClearFaults.code(), 0x03);
        assert_eq!(CommandCode::VoutMode.code(), 0x20);
        assert_eq!(CommandCode::StatusWord.code(), 0x79);
        assert_eq!(CommandCode::ReadVin.code(), 0x88);
        assert_eq!(CommandCode::ReadVout.code(), 0x8B);
        assert_eq!(CommandCode::MfrId.code(), 0x99);
        assert_eq!(CommandCode::UserData00.code(), 0xB0);
        assert_eq!(CommandCode::UserData15.code(), 0xBF);
        assert_eq!(CommandCode::PmbusCommandExt.code(), 0xFF);
    }

    #[test]
    fn from_u8() {
        let code: u8 = CommandCode::ReadPout.into();
        assert_eq!(code, 0x96);
    }
}
