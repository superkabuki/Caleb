//! src/short.rs

use num_bigint::{self, BigInt, Sign};

pub struct ShortBit {
    idx: usize,
    bit_size: usize,
    bits: BigInt,
}

/// ShortBit takes a byte array and converts it into a large integer.
impl ShortBit {
    pub fn new(bytes: &[u8]) -> ShortBit {
        Self {
            idx: bytes.len() << 3,
            bit_size: bytes.len() << 3,
            bits: BigInt::from_bytes_be(Sign::Plus, &bytes),
        }
    }

    pub fn get_idx(&self) -> usize {
        self.idx
    }

    /// starting at `self.idx` of `self.bits`, slice off `num_bits` of bits.
    pub fn as_int(&mut self, num_bits: usize) -> BigInt {
        if self.idx < num_bits {
            return BigInt::ZERO;
        }
        self.forward(num_bits);
        &self.bits >> self.idx & !(!BigInt::ZERO << num_bits)
    }

    /// returns the hex value of `num_bits` of bits
    pub fn as_hex(&mut self, num_bits: usize) -> String {
        let hex = format!("{:x}", self.as_int(num_bits));
        format!("0x{}{}", ["", "0"][hex.len() & 1], hex)
    }

    /// returns `num_bits` of bits as bytes
    pub fn as_bytes(&mut self, num_bits: usize) -> Vec<u8> {
        BigInt::to_bytes_be(&self.as_int(num_bits)).1
    }

    /// returns one bit as `true` or `false`
    pub fn as_flag(&mut self) -> bool {
        self.as_int(1) == BigInt::from(1)
    }

    /// advances the start point forward by `num_bits`
    pub fn forward(&mut self, num_bits: usize) {
        self.idx -= num_bits;
    }
}
