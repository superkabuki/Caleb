//! src/lib.rs

pub mod info;
pub mod short;

#[cfg(test)]
mod tests {
    use crate::short::ShortBit;
    use num_bigint::BigInt;

    #[test]
    fn test_short_bit() {
        let mut sb = ShortBit::new("lefthandtoGod".as_bytes());

        assert_eq!(sb.as_int(8), Some(BigInt::from(108)));
        assert_eq!(sb.as_flag(None), false);
        assert_eq!(sb.as_int(7), Some(BigInt::from(101)));
        assert_eq!(sb.as_bytes(32), "ftha".as_bytes());
        assert_eq!(sb.as_hex(17), "0xdcc8");
    }

    #[test]
    fn test_splice_info_section() {}
}
