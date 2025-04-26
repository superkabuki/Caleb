//! src/lib.rs

pub mod info;
pub mod info2;
pub mod short;
pub mod timesignal;

#[cfg(test)]
mod tests {
    use crate::short::ShortBit;
    use num_bigint::BigInt;

    #[test]
    fn test_short_bit() {
        let mut sb = ShortBit::new("lefthandtoGod".as_bytes());

        assert_eq!(sb.as_int(8), Some(BigInt::from(108)));
        assert_eq!(sb.as_flag(None), Some(false));
        assert_eq!(sb.as_int(7), Some(BigInt::from(101)));
        assert_eq!(sb.as_bytes(32), Some("ftha".as_bytes().to_vec()));
        assert_eq!(sb.as_hex(17), Some("0xdcc8".to_string()));
    }

    #[test]
    fn test_splice_info_section() {
        // NO TESTS
    }

    #[test]
    fn test_time_signal() {
        // NO TESTS
    }
}
