//! src/info.rs

//#![allow(unused)]

use std::collections::HashMap;

use num_bigint::BigInt;

use crate::short::{SbType, ShortBit};

/// This is not great but for now it works
static SAPS: [&'static str; 4] = [
    "Type 1 Closed GOP with no leading pictures",
    "Type 2 Closed GOP with leading pictures",
    "Type 3 Open GOP",
    "No Sap Type",
];

pub enum SIType {
    Int(BigInt),
    Hex(String),
    Flag(bool),
    Bytes(Vec<u8>),
    SapType(String),
    TimeStamp(f64),
    None,
}

/// The SCTE-35 splice info section
pub struct SpliceInfoSection {
    spliced_info: HashMap<String, SIType>,
}

impl SpliceInfoSection {
    pub fn new() -> SpliceInfoSection {
        Self {
            spliced_info: HashMap::new(),
        }
    }

    /// ticks to 90k timestamps
    fn as_90k(&self, int_time: u64) -> f64 {
        let sig_figs = 6;
        let p = u64::pow(10, sig_figs);
        ((int_time * p) as f64 / 90000.0).round() / p as f64
    }

    /// excludes None values, map all to string
    fn kv_clean(&self) -> HashMap<String, String> {
        self.spliced_info
            .iter()
            .filter_map(|(k, v)| match v {
                SIType::Int(i) => Some((k.to_owned(), format!("{}", i))),
                SIType::Hex(h) => Some((k.to_owned(), h.to_owned())),
                SIType::Flag(f) => Some((k.to_owned(), format!("{}", f))),
                SIType::Bytes(b) => {
                    if b.len() == 0 {
                        return None;
                    }
                    Some((k.to_owned(), format!("{:?}", b)))
                }
                SIType::SapType(s) => Some((k.to_owned(), s.to_owned())),
                SIType::TimeStamp(t) => Some((k.to_owned(), format!("{}", t))),
                _ => None,
            })
            .collect()
    }

    /// returns instance as a `kv_clean`ed hashmap
    pub fn get(&self) -> HashMap<String, String> {
        self.kv_clean()
    }

    /// get sap details from `SAPS`
    fn get_sap(&self, sap_type: SbType) -> SIType {
        let st = match sap_type {
            SbType::Hex(h) => h,
            _ => String::new(),
        };
        let hex = st.trim_start_matches("0x").to_string();
        let idx = usize::from_str_radix(&hex, 16).expect("Could not convert hex value to usize");
        SIType::SapType(SAPS[idx].to_string())
    }

    /// get pts adjustment with `self.as_90k`
    fn get_pts(&self, ticks: SbType) -> SIType {
        let t = match ticks {
            SbType::Int(i) => i,
            _ => BigInt::ZERO,
        };
        let pts_adjustment_ticks = t.to_u64_digits().1[0];
        SIType::TimeStamp(self.as_90k(pts_adjustment_ticks))
    }

    /// converts `SbType` enums into `SIType`
    fn map_sb_si(&self, v: &SbType) -> SIType {
        match v {
            SbType::Hex(h) => SIType::Hex(h.clone()),
            SbType::Int(i) => SIType::Int(i.clone()),
            SbType::Flag(f) => SIType::Flag(f.clone()),
            SbType::Bytes(b) => SIType::Bytes(b.clone()),
            SbType::None => SIType::None,
        }
    }

    /// InfoSection.decode
    pub fn decode(&mut self, bites: &[u8]) {
        let mut sb = ShortBit::new(bites);
        self.spliced_info.extend(
            [
                ("table_id", sb.as_hex(8)),
                ("section_syntax_indicator", sb.as_flag(Some(1))),
                ("private", sb.as_flag(Some(1))),
            ]
            .map(|v| (v.0.to_string(), self.map_sb_si(&v.1))),
        );

        // sap type
        let sap_type = sb.as_hex(8);

        self.spliced_info.extend(
            [
                ("sap_type", self.map_sb_si(&sap_type)),
                ("sap_details", self.get_sap(sap_type)),
            ]
            .map(|v| (v.0.to_string(), v.1)),
        );

        self.spliced_info.extend(
            [
                ("section_length", sb.as_int(12)),
                ("protocol_version", sb.as_int(8)),
                ("encrypted_packet", sb.as_flag(Some(1))),
                ("encryption_algorithm", sb.as_int(6)),
            ]
            .map(|v| (v.0.to_string(), self.map_sb_si(&v.1))),
        );
        self.spliced_info
            .insert("pts_adjustment".to_string(), self.get_pts(sb.as_int(33)));
        self.spliced_info.extend(
            [
                ("cw_index", sb.as_hex(8)),
                ("tier", sb.as_hex(8)),
                ("splice_command_length", sb.as_int(8)),
                ("splice_command_type", sb.as_int(8)),
            ]
            .map(|v| (v.0.to_string(), self.map_sb_si(&v.1))),
        );
    }
}
