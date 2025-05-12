//! src/info2.rs

#![allow(unused)]

use num_bigint::BigInt;

use crate::json::{self, CleanJson, Number};
use crate::short::ShortBit;
// use crate::timesignal::TimeSignal;
use std::collections::BTreeMap;

// This is not great but for now it works
static SAPS: [&'static str; 4] = [
    "Type 1 Closed GOP with no leading pictures",
    "Type 2 Closed GOP with leading pictures",
    "Type 3 Open GOP",
    "No Sap Type",
];

/// The SCTE-35 splice info section
#[derive(Clone, Debug)]
pub struct SpliceInfoSection {
    table_id: CleanJson,                 // hex
    section_syntax_indicator: CleanJson, // flag
    private: CleanJson,                  // flag
    sap_type: CleanJson,                 // hex
    sap_details: CleanJson,              // string (saps)
    section_length: CleanJson,           // int
    protocol_version: CleanJson,         // int
    encrypted_packet: CleanJson,         // flag
    encryption_algorithm: CleanJson,     // int
    pts_adjustment: CleanJson,           // f64
    cw_index: CleanJson,                 // hex
    tier: CleanJson,                     // hex
    splice_command_length: CleanJson,    // int
    splice_command_type: CleanJson,      // int
    descriptor_loop_length: CleanJson,   // int
    crc: CleanJson,                      // hex
}

impl SpliceInfoSection {
    pub fn new() -> SpliceInfoSection {
        Self {
            table_id: CleanJson::Null,
            section_syntax_indicator: CleanJson::Null,
            private: CleanJson::Null,
            sap_type: CleanJson::Null,
            sap_details: CleanJson::Null,
            section_length: CleanJson::Null,
            protocol_version: CleanJson::Null,
            encrypted_packet: CleanJson::Null,
            encryption_algorithm: CleanJson::Null,
            pts_adjustment: CleanJson::Null,
            cw_index: CleanJson::Null,
            tier: CleanJson::Null,
            splice_command_length: CleanJson::Null,
            splice_command_type: CleanJson::Null,
            descriptor_loop_length: CleanJson::Null,
            crc: CleanJson::Null,
        }
    }

    /// ticks to 90k timestamps
    fn as_90k(&self, int_time: u64) -> f64 {
        let sig_figs = 6;
        let p = u64::pow(10, sig_figs);
        ((int_time * p) as f64 / 90000.0).round() / p as f64
    }

    /// returns json struct, removing bad values
    fn kv_clean(&self) -> BTreeMap<String, CleanJson> {
        fn rec_clean(val: CleanJson) -> CleanJson {
            match val {
                CleanJson::Array(a) => {
                    CleanJson::Array(a.iter().map(|v| rec_clean(v.to_owned())).collect())
                }
                CleanJson::Object(o) => CleanJson::Object(
                    o.iter()
                        .map(|(k, v)| (k.to_owned(), rec_clean(v.clone())))
                        .collect(),
                ),
                _ => val,
            }
        }
        self.iter()
            .filter_map(|(k, v)| {
                let val = rec_clean(v.clone());
                if let CleanJson::Null = val {
                    None
                } else {
                    Some((k.to_owned(), val.to_owned()))
                }
            })
            .collect()
    }

    /// returns instance as a `kv_clean`ed map
    pub fn get(&self) -> BTreeMap<String, CleanJson> {
        self.kv_clean()
    }

    /// returns self as `kv_clean`ed json
    pub fn json(&self) -> String {
        json::to_json(self.kv_clean())
    }

    /// show prints self as formated json to stderr
    pub fn show(&self) {
        println!("{}", json::to_pretty(self.json(), 4));
    }

    /// get sap details from `SAPS`
    fn get_sap_details(&self, sap_type: String) -> String {
        let hex = sap_type.trim_start_matches("0x").to_string();
        let idx = usize::from_str_radix(&hex, 16).expect("Could not convert hex value to usize");
        SAPS[idx].to_string()
    }

    /// InfoSection.decode
    pub fn decode(&mut self, bytes: &[u8]) {
        let mut shortb = ShortBit::new(bytes);
        self.table_id = CleanJson::String(shortb.as_hex(8));
        self.section_syntax_indicator = CleanJson::Bool(shortb.as_flag());
        self.private = CleanJson::Bool(shortb.as_flag());
        let sap_type = shortb.as_hex(2);
        self.sap_type = CleanJson::String(sap_type.clone());
        self.sap_details = CleanJson::String(self.get_sap_details(sap_type));
        self.section_length = CleanJson::Number(Number::BigInt(shortb.as_int(12)));
        self.protocol_version = CleanJson::Number(Number::BigInt(shortb.as_int(8)));
        self.encrypted_packet = CleanJson::Bool(shortb.as_flag());
        self.encryption_algorithm = CleanJson::Number(Number::BigInt(shortb.as_int(6)));
        let bs = shortb.as_int(33).to_u64_digits().1;
        self.pts_adjustment = CleanJson::Number(Number::Float(self.as_90k(if bs.len() > 0 {
            bs[0]
        } else {
            0
        })));
        self.cw_index = CleanJson::String(shortb.as_hex(8));
        self.tier = CleanJson::String(shortb.as_hex(12));
        self.splice_command_length = CleanJson::Number(Number::BigInt(shortb.as_int(12)));
        self.splice_command_type = CleanJson::Number(Number::BigInt(shortb.as_int(8)));
    }

    /// iterate through struct fields
    fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self,
            index: 0,
        }
    }
}

struct Iter<'a> {
    inner: &'a SpliceInfoSection,
    index: u8,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a CleanJson);
    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => ("table_id", &self.inner.table_id),
            1 => (
                "section_syntax_indicator",
                &self.inner.section_syntax_indicator,
            ),
            2 => ("private", &self.inner.private),
            3 => ("sap_type", &self.inner.sap_type),
            4 => ("sap_details", &self.inner.sap_details),
            5 => ("section_length", &self.inner.section_length),
            6 => ("protocol_cersion", &self.inner.protocol_version),
            7 => ("encrypted_packet", &self.inner.encrypted_packet),
            8 => ("encryption_algorithm", &self.inner.encryption_algorithm),
            9 => ("pts_adjustment", &self.inner.pts_adjustment),
            10 => ("cw_index", &self.inner.cw_index),
            11 => ("tier", &self.inner.tier),
            12 => ("splice_command_length", &self.inner.splice_command_length),
            13 => ("splice_command_type", &self.inner.splice_command_type),
            14 => ("descriptor_loop_length", &self.inner.descriptor_loop_length),
            15 => ("crc", &self.inner.crc),
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}
