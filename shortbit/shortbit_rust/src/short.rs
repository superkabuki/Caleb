//! src/short.rs

#![allow(unused)]

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

    /// starting at `self.idx` of `self.bits`, slice off `num_bits` of bits.
    pub fn as_int(&mut self, num_bits: usize) -> Option<BigInt> {
        if self.idx >= num_bits {
            self.forward(num_bits);
            return Some(&self.bits >> self.idx & !(!BigInt::ZERO << num_bits));
        }
        None
    }

    /// returns the hex value of `num_bits` of bits
    pub fn as_hex(&mut self, num_bits: usize) -> Option<String> {
        let hex = match self.as_int(num_bits) {
            Some(b) => format!("{:x}", b),
            None => return None,
        };
        Some(format!("0x{}{}", ["", "0"][hex.len() & 1], hex))
    }

    /// returns `num_bits` of bits as bytes
    pub fn as_bytes(&mut self, num_bits: usize) -> Option<Vec<u8>> {
        match self.as_int(num_bits) {
            Some(i) => Some(BigInt::to_bytes_be(&i).1),
            None => None,
        }
    }

    /// returns one bit as `true` or `false`
    pub fn as_flag(&mut self, num_bits: Option<usize>) -> Option<bool> {
        match self.as_int(num_bits.unwrap_or(1)) {
            Some(b) => Some(b & BigInt::from(1) == BigInt::from(1)),
            None => None,
        }
    }

    /// advances the start point forward by `num_bits`
    fn forward(&mut self, num_bits: usize) {
        self.idx -= num_bits;
    }
}
