/// Const lookup table for 10^R where R is in [-8, 8].
const POW10: [f32; 17] = [
    1e-8, 1e-7, 1e-6, 1e-5, 1e-4, 1e-3, 1e-2, 1e-1, 1.0, 1e1, 1e2, 1e3, 1e4, 1e5, 1e6, 1e7, 1e8,
];

/// Return 10^r for r in [-8, 8]. Returns `None` if out of range.
fn pow10(r: i8) -> Option<f32> {
    let idx = (r as i16 + 8) as usize;
    POW10.get(idx).copied()
}

/// no_std-compatible rounding (round half away from zero).
fn round_f32(x: f32) -> f32 {
    if x >= 0.0 {
        (x + 0.5) as i32 as f32
    } else {
        (x - 0.5) as i32 as f32
    }
}

/// PMBus LINEAR11 data format.
///
/// Encodes a value as `Y * 2^N` where Y is an 11-bit signed mantissa
/// and N is a 5-bit signed exponent. Used for most PMBus telemetry
/// values (current, power, temperature, etc.).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Linear11(u16);

impl Linear11 {
    /// Construct from a raw 16-bit bus value.
    pub fn from_raw(raw: u16) -> Self {
        Self(raw)
    }

    /// Return the raw 16-bit value.
    pub fn raw(self) -> u16 {
        self.0
    }

    /// Decode to `f32`. Value = Y * 2^N.
    pub fn to_f32(self) -> f32 {
        let n = ((self.0 >> 11) as i8) << 3 >> 3; // sign-extend 5 bits
        let y = ((self.0 & 0x07FF) as i16) << 5 >> 5; // sign-extend 11 bits
        (y as f32) * exp2f(n as i32)
    }

    /// Encode an `f32` value into LINEAR11 format.
    ///
    /// Returns `None` if the value cannot be represented (e.g., too large).
    pub fn from_f32(value: f32) -> Option<Self> {
        if !value.is_finite() {
            return None;
        }

        if value == 0.0 {
            return Some(Self(0));
        }

        // Find the best exponent N such that Y = value / 2^N fits in [-1024, 1023].
        let mut best_n: i8 = 0;
        let mut best_y: i16 = 0;
        let mut best_err: f32 = f32::MAX;

        for n in -16i8..=15 {
            let y_f = value / exp2f(n as i32);
            let y_rounded = round_f32(y_f) as i32;
            if !(-1024..=1023).contains(&y_rounded) {
                continue;
            }
            let y = y_rounded as i16;
            let reconstructed = (y as f32) * exp2f(n as i32);
            let err = (value - reconstructed).abs();
            if err < best_err {
                best_err = err;
                best_n = n;
                best_y = y;
            }
        }

        if best_err == f32::MAX {
            return None;
        }

        let n_bits = (best_n as u16) & 0x1F;
        let y_bits = (best_y as u16) & 0x07FF;
        Some(Self((n_bits << 11) | y_bits))
    }
}

/// PMBus ULINEAR16 data format.
///
/// Used for output voltage. Encodes as `V * 2^N` where V is a 16-bit
/// unsigned value and N (the exponent) comes from the VOUT_MODE register.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ULinear16(u16);

impl ULinear16 {
    /// Construct from a raw 16-bit bus value.
    pub fn from_raw(raw: u16) -> Self {
        Self(raw)
    }

    /// Return the raw 16-bit value.
    pub fn raw(self) -> u16 {
        self.0
    }

    /// Decode to `f32` given the exponent from VOUT_MODE.
    pub fn to_f32(self, exponent: i8) -> f32 {
        (self.0 as f32) * exp2f(exponent as i32)
    }

    /// Encode an `f32` into ULINEAR16 given the exponent from VOUT_MODE.
    ///
    /// Returns `None` if the value cannot be represented.
    pub fn from_f32(value: f32, exponent: i8) -> Option<Self> {
        if !value.is_finite() || value < 0.0 {
            return None;
        }
        let raw_f = value / exp2f(exponent as i32);
        let raw_rounded = round_f32(raw_f) as u32;
        if raw_rounded > 0xFFFF {
            return None;
        }
        Some(Self(raw_rounded as u16))
    }
}

/// PMBus DIRECT data format coefficients.
///
/// Converts between raw register values and real-world units using:
/// - Decode: `X = (1/m) * (Y * 10^(-R) - b)`
/// - Encode: `Y = (m * X + b) * 10^R`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectCoefficients {
    pub m: i16,
    pub b: i16,
    pub r: i8,
}

impl DirectCoefficients {
    /// Create new coefficients.
    pub fn new(m: i16, b: i16, r: i8) -> Self {
        Self { m, b, r }
    }

    /// Decode a raw register value to an `f32`.
    pub fn to_f32(self, raw: i16) -> f32 {
        let scale = pow10(-self.r).unwrap_or(1.0);
        (1.0 / self.m as f32) * ((raw as f32) * scale - self.b as f32)
    }

    /// Encode an `f32` value to a raw register value.
    ///
    /// Returns `None` if the result doesn't fit in i16.
    pub fn from_f32(self, value: f32) -> Option<i16> {
        let scale = pow10(self.r).unwrap_or(1.0);
        let y_f = (self.m as f32 * value + self.b as f32) * scale;
        let y = round_f32(y_f) as i32;
        if y < i16::MIN as i32 || y > i16::MAX as i32 {
            return None;
        }
        Some(y as i16)
    }

    /// Parse a 5-byte COEFFICIENTS response (from command 0x30).
    ///
    /// Format: `[m_low, m_high, b_low, b_high, r]`
    pub fn from_coefficients_response(data: &[u8]) -> Option<Self> {
        if data.len() < 5 {
            return None;
        }
        let m = i16::from_le_bytes([data[0], data[1]]);
        let b = i16::from_le_bytes([data[2], data[3]]);
        let r = data[4] as i8;
        Some(Self { m, b, r })
    }
}

/// Compute 2^n for integer n using bit shifts and division.
fn exp2f(n: i32) -> f32 {
    if (0..31).contains(&n) {
        (1u32 << n) as f32
    } else if n < 0 && n > -31 {
        1.0 / (1u32 << (-n)) as f32
    } else if n >= 31 {
        f32::MAX
    } else {
        f32::MIN_POSITIVE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn linear11_decode() {
        // Example: 12.5A encoded as N=-1, Y=25 → raw = (0x1F << 11) | 25 = 0xF819
        let raw = (0x1Fu16 << 11) | 25;
        let val = Linear11::from_raw(raw);
        let f = val.to_f32();
        assert!((f - 12.5).abs() < 0.01, "expected 12.5, got {f}");
    }

    #[test]
    fn linear11_encode_decode_roundtrip() {
        for &v in &[0.0, 1.0, -1.0, 12.5, 100.0, 0.125, -500.0, 1023.0] {
            if let Some(l) = Linear11::from_f32(v) {
                let decoded = l.to_f32();
                let err = (v - decoded).abs();
                let tolerance = v.abs() * 0.01 + 0.01;
                assert!(err < tolerance, "roundtrip failed for {v}: got {decoded}");
            }
        }
    }

    #[test]
    fn linear11_zero() {
        let l = Linear11::from_f32(0.0).unwrap();
        assert_eq!(l.raw(), 0);
        assert_eq!(l.to_f32(), 0.0);
    }

    #[test]
    fn linear11_nan_returns_none() {
        assert!(Linear11::from_f32(f32::NAN).is_none());
        assert!(Linear11::from_f32(f32::INFINITY).is_none());
    }

    #[test]
    fn ulinear16_decode() {
        // Example: exponent = -13, raw = 0x2000 → V = 8192 * 2^-13 = 1.0V
        let val = ULinear16::from_raw(0x2000);
        let f = val.to_f32(-13);
        assert!((f - 1.0).abs() < 0.001, "expected 1.0, got {f}");
    }

    #[test]
    fn ulinear16_roundtrip() {
        let exponent: i8 = -13;
        for &v in &[0.0, 1.0, 1.2, 3.3, 5.0] {
            if let Some(u) = ULinear16::from_f32(v, exponent) {
                let decoded = u.to_f32(exponent);
                let err = (v - decoded).abs();
                let tolerance = 0.001;
                assert!(err < tolerance, "roundtrip failed for {v}: got {decoded}");
            }
        }
    }

    #[test]
    fn ulinear16_negative_returns_none() {
        assert!(ULinear16::from_f32(-1.0, -13).is_none());
    }

    #[test]
    fn direct_coefficients_decode() {
        // Example: m=1, b=0, R=0 → identity
        let c = DirectCoefficients::new(1, 0, 0);
        assert_eq!(c.to_f32(100), 100.0);
    }

    #[test]
    fn direct_coefficients_encode() {
        let c = DirectCoefficients::new(1, 0, 0);
        assert_eq!(c.from_f32(100.0), Some(100));
    }

    #[test]
    fn direct_coefficients_with_offset() {
        // m=10, b=5, R=0 → Y = 10*X + 5, X = (Y - 5) / 10
        let c = DirectCoefficients::new(10, 5, 0);
        let raw = c.from_f32(3.0).unwrap(); // Y = 10*3 + 5 = 35
        assert_eq!(raw, 35);
        let decoded = c.to_f32(35); // X = (35 - 5) / 10 = 3.0
        assert!((decoded - 3.0).abs() < 0.01);
    }

    #[test]
    fn direct_coefficients_from_response() {
        let data = [0x0A, 0x00, 0x05, 0x00, 0x00]; // m=10, b=5, R=0
        let c = DirectCoefficients::from_coefficients_response(&data).unwrap();
        assert_eq!(c.m, 10);
        assert_eq!(c.b, 5);
        assert_eq!(c.r, 0);
    }

    #[test]
    fn direct_coefficients_short_response_returns_none() {
        assert!(DirectCoefficients::from_coefficients_response(&[1, 2, 3]).is_none());
    }

    #[test]
    fn pow10_table() {
        assert!((pow10(0).unwrap() - 1.0).abs() < f32::EPSILON);
        assert!((pow10(1).unwrap() - 10.0).abs() < f32::EPSILON);
        assert!((pow10(-1).unwrap() - 0.1).abs() < 0.001);
        assert!((pow10(3).unwrap() - 1000.0).abs() < f32::EPSILON);
        assert!(pow10(9).is_none());
        assert!(pow10(-9).is_none());
    }

    #[test]
    fn test_from_linear11_tps546() {
        assert_eq!(Linear11::from_raw(0).to_f32(), 0.0);
        assert_eq!(Linear11::from_raw(0xF0D0).to_f32(), 52.0); // IOUT_OC_FAULT_LIMIT
        assert_eq!(Linear11::from_raw(0xE340).to_f32(), 52.0); // same value, different encoding
        assert_eq!(Linear11::from_raw(0xF0A0).to_f32(), 40.0); // IOUT_OC_WARN_LIMIT
        assert_eq!(Linear11::from_raw(0xC840).to_f32(), 0.5); // VOUT_SCALE_LOOP
        assert_eq!(Linear11::from_raw(0xC880).to_f32(), 1.0); // IOUT_CAL_GAIN
    }

    #[test]
    fn test_to_linear11_tps546() {
        for &v in &[0.0, 52.0, 40.0, 0.5, 1.0] {
            let encoded = Linear11::from_f32(v).unwrap();
            assert_eq!(encoded.to_f32(), v, "roundtrip failed for {v}");
        }
    }

    #[test]
    fn test_from_ulinear16_tps546() {
        let exp: i8 = -12;
        // 1229 * 2^-12 = 0.300048828125 V
        let v = ULinear16::from_raw(1229).to_f32(exp);
        assert!((v - 0.300048828125).abs() < 1e-9);
        // 0x333 (819) * 2^-12 = 0.199951171875 V
        let v = ULinear16::from_raw(0x333).to_f32(exp);
        assert!((v - 0.199951171875).abs() < 1e-9);
        // 0x4b4 (1204) * 2^-12 = 0.29394531250 V
        let v = ULinear16::from_raw(0x4b4).to_f32(exp);
        assert!((v - 0.29394531250).abs() < 1e-9);
    }

    #[test]
    fn test_to_ulinear16_tps546() {
        let exp: i8 = -12;
        assert_eq!(ULinear16::from_f32(0.0, exp).unwrap().raw(), 0);
        // 300mV = 0.300V -> round(0.300 / 2^-12) = round(1228.8) = 1229
        assert_eq!(ULinear16::from_f32(0.300, exp).unwrap().raw(), 1229);
        // 200mV = 0.200V -> round(0.200 / 2^-12) = round(819.2) = 819 = 0x333
        assert_eq!(ULinear16::from_f32(0.200, exp).unwrap().raw(), 0x333);
        // 294mV = 0.294V -> round(0.294 / 2^-12) = round(1204.2) = 1204 = 0x4b4
        assert_eq!(ULinear16::from_f32(0.294, exp).unwrap().raw(), 0x4b4);
        // 700mV = 0.700V -> round(0.700 / 2^-12) = round(2867.2) = 2867
        assert_eq!(ULinear16::from_f32(0.700, exp).unwrap().raw(), 2867);
    }
}
