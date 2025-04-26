//! src/short.rs

#![allow(unused)]

use num_bigint::{self, BigInt, Sign};

pub struct ShortBit {
    idx: usize,
    bit_size: usize,
    bits: BigInt,
}

#[derive(Debug, PartialEq)]
pub enum SbType {
    Int(BigInt),
    Hex(String),
    Flag(bool),
    Bytes(Vec<u8>),
    None,
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
    pub fn as_int(&mut self, num_bits: usize) -> SbType {
        if self.idx >= num_bits {
            self.forward(num_bits);
            return SbType::Int(&self.bits >> self.idx & !(!BigInt::ZERO << num_bits));
        }
        SbType::None
    }

    /// returns the hex value of `num_bits` of bits
    pub fn as_hex(&mut self, num_bits: usize) -> SbType {
        let hex = match self.as_int(num_bits) {
            SbType::Int(b) => format!("{:x}", b),
            _ => return SbType::None,
        };
        SbType::Hex(format!("0x{}{}", ["", "0"][hex.len() & 1], hex))
    }

    /// returns `num_bits` of bits as bytes
    pub fn as_bytes(&mut self, num_bits: usize) -> SbType {
        match self.as_int(num_bits) {
            SbType::Int(i) => SbType::Bytes(BigInt::to_bytes_be(&i).1),
            _ => SbType::None,
        }
    }

    /// returns one bit as `true` or `false`
    pub fn as_flag(&mut self, num_bits: Option<usize>) -> SbType {
        match self.as_int(num_bits.unwrap_or(1)) {
            SbType::Int(b) => SbType::Flag(b & BigInt::from(1) == BigInt::from(1)),
            _ => SbType::None,
        }
    }

    /// advances the start point forward by `num_bits`
    fn forward(&mut self, num_bits: usize) {
        self.idx -= num_bits;
    }
}
