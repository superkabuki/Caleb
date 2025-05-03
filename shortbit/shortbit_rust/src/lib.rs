//! src/lib.rs

pub mod info;
pub mod info2;
pub mod json;
pub mod short;
pub mod timesignal;

#[cfg(test)]
mod tests {
    use crate::{
        json::{CleanJson, Number},
        short::{SbType, ShortBit},
    };
    use num_bigint::BigInt;
    use std::collections::BTreeMap;

    #[test]
    fn test_short_bit() {
        let mut sb = ShortBit::new("lefthandtoGod".as_bytes());

        assert_eq!(sb.as_int(8), SbType::Int(BigInt::from(108)));
        assert_eq!(sb.as_flag(None), SbType::Flag(false));
        assert_eq!(sb.as_int(7), SbType::Int(BigInt::from(101)));
        assert_eq!(sb.as_bytes(32), SbType::Bytes("ftha".as_bytes().to_vec()));
        assert_eq!(sb.as_hex(17), SbType::Hex("0xdcc8".to_string()));
    }

    #[test]
    fn json_conversion() {
        use super::*;
        let data = BTreeMap::from(
            [
                ("name", CleanJson::String(String::from("foo"))),
                ("is_active", CleanJson::Bool(false)),
                (
                    "arr",
                    CleanJson::Array(Vec::from([
                        CleanJson::Bool(true),
                        CleanJson::Bool(false),
                        CleanJson::Object(BTreeMap::from(
                            [
                                ("idk_what6", CleanJson::Bool(true)),
                                ("number243", CleanJson::Number(Number::Int(8))),
                                ("letters", CleanJson::String(String::from("ewtoa"))),
                            ]
                            .map(|(k, v)| (String::from(k.to_owned()), v)),
                        )),
                    ])),
                ),
                ("idk_what", CleanJson::Bool(true)),
                ("number", CleanJson::Number(Number::Int(8))),
                ("number2", CleanJson::Number(Number::Float(8213.43982))),
            ]
            .map(|(k, v)| (String::from(k.to_owned()), v)),
        );
        let converted = json::to_json(data);
        println!("{}", converted);
        assert_eq!(converted, r#"{"arr":[true,false,{"idk_what6":true,"letters":"ewtoa","number243":8}],"idk_what":true,"is_active":false,"name":"foo","number":8,"number2":8213.43982}"#.to_string());
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
