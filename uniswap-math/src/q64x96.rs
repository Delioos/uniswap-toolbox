/// Represents a Q64.96 fixed-point number (also know as a Q number)

/// Q numbers consist of a 64-bit integer part and a 96-bit fractional part.
/// The 64-bit integer part is the most significant part, and the 96-bit fractional part is the least significant part.
/// 
/// Uniswap uses them to mimic floating point numbers to avoid precision loss due to evm non floating point operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Q64x96(i128);

impl Q64x96 {

    pub fn from_float(f: f64) -> Self {
        let bits = f.to_bits();
        let high = (bits >> 32) as i64;
        let low = bits as i64;
        Q64x96((high as i128) << 64 | (low as i128))
    }

    pub fn to_float(self) -> f64 {
        let high = (self.0 >> 64) as u64;
        let low = (self.0 & 0xFFFFFFFFFFFFFFFF) as u64;
        f64::from_bits((high << 32) | low)
    }
} 