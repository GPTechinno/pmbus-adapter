/// The VOUT_MODE data format type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoutModeType {
    /// Linear mode — bits\[4:0\] contain a signed 5-bit exponent for ULINEAR16.
    ULinear16 { exponent: i8 },
    /// VID mode — bits\[4:0\] identify the VID code table.
    Vid { code: u8 },
    /// Direct format mode — coefficients come from COEFFICIENTS command.
    Direct,
    /// IEEE 754 half-precision floating point.
    IeeeHalf,
}

/// Parsed VOUT_MODE register (command 0x20).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VoutMode {
    /// Bit 7: when set, VOUT_COMMAND is a relative margin.
    pub relative: bool,
    /// The output voltage encoding mode.
    pub mode: VoutModeType,
}

impl VoutMode {
    /// Parse a raw VOUT_MODE register byte.
    pub fn from_raw(raw: u8) -> Self {
        let relative = (raw & 0x80) != 0;
        let mode_bits = (raw >> 5) & 0x03;
        let param = raw & 0x1F;

        let mode = match mode_bits {
            0b00 => {
                // Sign-extend the 5-bit exponent
                let exponent = ((param as i8) << 3) >> 3;
                VoutModeType::ULinear16 { exponent }
            }
            0b01 => VoutModeType::Vid { code: param },
            0b10 => VoutModeType::Direct,
            _ => VoutModeType::IeeeHalf,
        };

        Self { relative, mode }
    }

    /// Encode back to a raw register byte.
    pub fn to_raw(self) -> u8 {
        let rel_bit = if self.relative { 0x80 } else { 0x00 };
        match self.mode {
            VoutModeType::ULinear16 { exponent } => {
                rel_bit | ((exponent as u8) & 0x1F)
            }
            VoutModeType::Vid { code } => rel_bit | (0b01 << 5) | (code & 0x1F),
            VoutModeType::Direct => rel_bit | (0b10 << 5),
            VoutModeType::IeeeHalf => rel_bit | (0b11 << 5),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ulinear16_negative_exponent() {
        // exponent = -13 → two's complement 5-bit = 0b10011 = 0x13
        let raw = 0x13;
        let mode = VoutMode::from_raw(raw);
        assert!(!mode.relative);
        assert_eq!(mode.mode, VoutModeType::ULinear16 { exponent: -13 });
        assert_eq!(mode.to_raw(), raw);
    }

    #[test]
    fn ulinear16_positive_exponent() {
        let raw = 0x03; // exponent = 3
        let mode = VoutMode::from_raw(raw);
        assert_eq!(mode.mode, VoutModeType::ULinear16 { exponent: 3 });
        assert_eq!(mode.to_raw(), raw);
    }

    #[test]
    fn vid_mode() {
        let raw = 0x21; // mode=01, code=1
        let mode = VoutMode::from_raw(raw);
        assert_eq!(mode.mode, VoutModeType::Vid { code: 1 });
        assert_eq!(mode.to_raw(), raw);
    }

    #[test]
    fn direct_mode() {
        let raw = 0x40; // mode=10
        let mode = VoutMode::from_raw(raw);
        assert_eq!(mode.mode, VoutModeType::Direct);
        assert_eq!(mode.to_raw(), raw);
    }

    #[test]
    fn ieee_half_mode() {
        let raw = 0x60; // mode=11
        let mode = VoutMode::from_raw(raw);
        assert_eq!(mode.mode, VoutModeType::IeeeHalf);
        assert_eq!(mode.to_raw(), raw);
    }

    #[test]
    fn relative_bit() {
        let raw = 0x93; // relative=1, mode=00, exponent=-13
        let mode = VoutMode::from_raw(raw);
        assert!(mode.relative);
        assert_eq!(mode.mode, VoutModeType::ULinear16 { exponent: -13 });
        assert_eq!(mode.to_raw(), raw);
    }

    #[test]
    fn roundtrip_all_modes() {
        for raw in 0u8..=255 {
            let mode = VoutMode::from_raw(raw);
            let mode_bits = (raw >> 5) & 0x03;
            match mode_bits {
                // ULinear16 and VID use all bits — exact roundtrip expected
                0b00 | 0b01 => {
                    assert_eq!(mode.to_raw(), raw, "roundtrip failed for raw=0x{raw:02X}");
                }
                // Direct and IeeeHalf have reserved lower bits — only upper bits roundtrip
                _ => {
                    assert_eq!(mode.to_raw() & 0xE0, raw & 0xE0, "mode roundtrip failed for raw=0x{raw:02X}");
                }
            }
        }
    }
}
