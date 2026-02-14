use bitflags::bitflags;

bitflags! {
    /// STATUS_BYTE register (0x78) — 8-bit summary status.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusByte: u8 {
        const BUSY                = 0x80;
        const OFF                 = 0x40;
        const VOUT_OV_FAULT       = 0x20;
        const IOUT_OC_FAULT       = 0x10;
        const VIN_UV_FAULT        = 0x08;
        const TEMPERATURE         = 0x04;
        const CML                 = 0x02;
        const NONE_OF_THE_ABOVE   = 0x01;
    }
}

bitflags! {
    /// STATUS_WORD register (0x79) — 16-bit extended status.
    ///
    /// Low byte matches STATUS_BYTE. High byte provides additional flags.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusWord: u16 {
        // Low byte (same as STATUS_BYTE)
        const BUSY                = 0x0080;
        const OFF                 = 0x0040;
        const VOUT_OV_FAULT       = 0x0020;
        const IOUT_OC_FAULT       = 0x0010;
        const VIN_UV_FAULT        = 0x0008;
        const TEMPERATURE         = 0x0004;
        const CML                 = 0x0002;
        const NONE_OF_THE_ABOVE   = 0x0001;
        // High byte
        const VOUT                = 0x8000;
        const IOUT_POUT           = 0x4000;
        const INPUT               = 0x2000;
        const MFR_SPECIFIC        = 0x1000;
        const POWER_GOOD_NEG      = 0x0800;
        const FANS                = 0x0400;
        const OTHER               = 0x0200;
        const UNKNOWN             = 0x0100;
    }
}

bitflags! {
    /// STATUS_VOUT register (0x7A).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusVout: u8 {
        const OV_FAULT            = 0x80;
        const OV_WARNING          = 0x40;
        const UV_WARNING          = 0x20;
        const UV_FAULT            = 0x10;
        const MAX_MIN_WARNING     = 0x08;
        const TON_MAX_FAULT       = 0x04;
        const TOFF_MAX_WARNING    = 0x02;
        const TRACKING_ERROR      = 0x01;
    }
}

bitflags! {
    /// STATUS_IOUT register (0x7B).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusIout: u8 {
        const OC_FAULT            = 0x80;
        const OC_LV_FAULT         = 0x40;
        const OC_WARNING          = 0x20;
        const UC_FAULT            = 0x10;
        const CURRENT_SHARE       = 0x08;
        const POWER_LIMITING      = 0x04;
        const POUT_OP_FAULT       = 0x02;
        const POUT_OP_WARNING     = 0x01;
    }
}

bitflags! {
    /// STATUS_INPUT register (0x7C).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusInput: u8 {
        const VIN_OV_FAULT        = 0x80;
        const VIN_OV_WARNING      = 0x40;
        const VIN_UV_WARNING      = 0x20;
        const VIN_UV_FAULT        = 0x10;
        const UNIT_OFF_LOW_VIN    = 0x08;
        const IIN_OC_FAULT        = 0x04;
        const IIN_OC_WARNING      = 0x02;
        const PIN_OP_WARNING      = 0x01;
    }
}

bitflags! {
    /// STATUS_TEMPERATURE register (0x7D).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusTemperature: u8 {
        const OT_FAULT            = 0x80;
        const OT_WARNING          = 0x40;
        const UT_WARNING          = 0x20;
        const UT_FAULT            = 0x10;
    }
}

bitflags! {
    /// STATUS_CML register (0x7E) — Communication, Memory, and Logic faults.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusCml: u8 {
        const INVALID_COMMAND     = 0x80;
        const INVALID_DATA        = 0x40;
        const PEC_FAILED          = 0x20;
        const MEMORY_FAULT        = 0x10;
        const PROCESSOR_FAULT     = 0x08;
        const COMM_FAULT_OTHER    = 0x02;
        const OTHER_MEM_LOGIC     = 0x01;
    }
}

bitflags! {
    /// STATUS_OTHER register (0x7F).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusOther: u8 {
        const BIT7 = 0x80;
        const BIT6 = 0x40;
        const BIT5 = 0x20;
        const BIT4 = 0x10;
        const BIT3 = 0x08;
        const BIT2 = 0x04;
        const BIT1 = 0x02;
        const BIT0 = 0x01;
    }
}

bitflags! {
    /// STATUS_FANS_1_2 register (0x81).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusFans12: u8 {
        const FAN1_FAULT          = 0x80;
        const FAN1_WARNING        = 0x40;
        const FAN1_SPEED_OVERRIDE = 0x20;
        const FAN2_FAULT          = 0x10;
        const FAN2_WARNING        = 0x08;
        const FAN2_SPEED_OVERRIDE = 0x04;
    }
}

bitflags! {
    /// STATUS_FANS_3_4 register (0x82).
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StatusFans34: u8 {
        const FAN3_FAULT          = 0x80;
        const FAN3_WARNING        = 0x40;
        const FAN3_SPEED_OVERRIDE = 0x20;
        const FAN4_FAULT          = 0x10;
        const FAN4_WARNING        = 0x08;
        const FAN4_SPEED_OVERRIDE = 0x04;
    }
}

// Convenience constructors for building from raw bus values.
impl StatusByte {
    pub fn from_raw(raw: u8) -> Self {
        Self::from_bits_truncate(raw)
    }
}

impl StatusWord {
    pub fn from_raw(raw: u16) -> Self {
        Self::from_bits_truncate(raw)
    }
}

impl StatusVout {
    pub fn from_raw(raw: u8) -> Self {
        Self::from_bits_truncate(raw)
    }
}

impl StatusIout {
    pub fn from_raw(raw: u8) -> Self {
        Self::from_bits_truncate(raw)
    }
}

impl StatusInput {
    pub fn from_raw(raw: u8) -> Self {
        Self::from_bits_truncate(raw)
    }
}

impl StatusTemperature {
    pub fn from_raw(raw: u8) -> Self {
        Self::from_bits_truncate(raw)
    }
}

impl StatusCml {
    pub fn from_raw(raw: u8) -> Self {
        Self::from_bits_truncate(raw)
    }
}

impl StatusOther {
    pub fn from_raw(raw: u8) -> Self {
        Self::from_bits_truncate(raw)
    }
}

impl StatusFans12 {
    pub fn from_raw(raw: u8) -> Self {
        Self::from_bits_truncate(raw)
    }
}

impl StatusFans34 {
    pub fn from_raw(raw: u8) -> Self {
        Self::from_bits_truncate(raw)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_byte_flags() {
        let s = StatusByte::from_raw(0x44);
        assert!(s.contains(StatusByte::OFF));
        assert!(s.contains(StatusByte::TEMPERATURE));
        assert!(!s.contains(StatusByte::BUSY));
    }

    #[test]
    fn status_word_high_byte() {
        let s = StatusWord::from_raw(0x8040);
        assert!(s.contains(StatusWord::VOUT));
        assert!(s.contains(StatusWord::OFF));
    }

    #[test]
    fn status_vout_flags() {
        let s = StatusVout::from_raw(0x90);
        assert!(s.contains(StatusVout::OV_FAULT));
        assert!(s.contains(StatusVout::UV_FAULT));
    }

    #[test]
    fn status_iout_flags() {
        let s = StatusIout::from_raw(0x81);
        assert!(s.contains(StatusIout::OC_FAULT));
        assert!(s.contains(StatusIout::POUT_OP_WARNING));
    }

    #[test]
    fn status_input_flags() {
        let s = StatusInput::from_raw(0xC0);
        assert!(s.contains(StatusInput::VIN_OV_FAULT));
        assert!(s.contains(StatusInput::VIN_OV_WARNING));
    }

    #[test]
    fn status_temperature_flags() {
        let s = StatusTemperature::from_raw(0xC0);
        assert!(s.contains(StatusTemperature::OT_FAULT));
        assert!(s.contains(StatusTemperature::OT_WARNING));
    }

    #[test]
    fn status_cml_flags() {
        let s = StatusCml::from_raw(0x80);
        assert!(s.contains(StatusCml::INVALID_COMMAND));
    }

    #[test]
    fn status_fans12_flags() {
        let s = StatusFans12::from_raw(0xC0);
        assert!(s.contains(StatusFans12::FAN1_FAULT));
        assert!(s.contains(StatusFans12::FAN1_WARNING));
    }

    #[test]
    fn status_fans34_flags() {
        let s = StatusFans34::from_raw(0x10);
        assert!(s.contains(StatusFans34::FAN4_FAULT));
    }

    #[test]
    fn status_empty() {
        assert!(StatusByte::from_raw(0).is_empty());
        assert!(StatusWord::from_raw(0).is_empty());
    }
}
