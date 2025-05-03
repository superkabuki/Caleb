//! src/info2.rs

//#![allow(unused)]

use crate::json::{CleanJson, Number};
use crate::short::{SbType, ShortBit};
use crate::timesignal::TimeSignal;
use num_bigint::BigInt;
use std::collections::BTreeMap;

/// This is not great but for now it works
static SAPS: [&'static str; 4] = [
    "Type 1 Closed GOP with no leading pictures",
    "Type 2 Closed GOP with leading pictures",
    "Type 3 Open GOP",
    "No Sap Type",
];

#[derive(Clone)]
pub enum SIType {
    Int(u64),
    Hex(String),
    Flag(bool),
    Bytes(Vec<u8>),
    SapType(String),
    TimeStamp(f64),
    // info.py implies this could be possible
    Map(BTreeMap<String, SIType>),
    // info.py implies this could be possible
    OtherArray(Vec<SIType>),
    SpliceInfoSection(Box<SpliceInfoSection>),
    TimeSignal(Box<TimeSignal>),
    // ...
    None,
}

/// The SCTE-35 splice info section
#[derive(Clone)]
pub struct SpliceInfoSection {
    table_id: SIType,                 // hex
    section_syntax_indicator: SIType, // flag
    private: SIType,                  // flag
    sap_type: SIType,                 // hex
    sap_details: SIType,              // string (saps)
    section_length: SIType,           // int
    protocol_version: SIType,         // int
    encrypted_packet: SIType,         // flag
    encryption_algorithm: SIType,     // int
    pts_adjustment: SIType,           // f64
    cw_index: SIType,                 // hex
    tier: SIType,                     // hex
    splice_command_length: SIType,    // int
    splice_command_type: SIType,      // int
    descriptor_loop_length: SIType,   // int
    crc: SIType,                      // hex
}

impl SpliceInfoSection {
    pub fn new() -> SpliceInfoSection {
        Self {
            table_id: SIType::None,
            section_syntax_indicator: SIType::None,
            private: SIType::None,
            sap_type: SIType::None,
            sap_details: SIType::None,
            section_length: SIType::None,
            protocol_version: SIType::None,
            encrypted_packet: SIType::None,
            encryption_algorithm: SIType::None,
            pts_adjustment: SIType::None,
            cw_index: SIType::None,
            tier: SIType::None,
            splice_command_length: SIType::None,
            splice_command_type: SIType::None,
            descriptor_loop_length: SIType::None,
            crc: SIType::None,
        }
    }

    /// ticks to 90k timestamps
    fn as_90k(&self, int_time: u64) -> f64 {
        let sig_figs = 6;
        let p = u64::pow(10, sig_figs);
        ((int_time * p) as f64 / 90000.0).round() / p as f64
    }

    fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self,
            index: 0,
        }
    }

    /**
    Will this handle recursion?
    CALEB: It does now. I've implemented it close to how info.py does it. I'm not a fan of it though:
    if I were to do this on my own I would try to avoid using recursion to begin with. That would be
    the rust way, at least. I understand that for a general json converter, recursion (or another
    method that accounts for an unknown amount of data) is difficult to avoid, but for this
    specific case I think it can be. I could be wrong: I don't know SCTE-35 that well.

    NOTE: I have not yet implemented a direct struct to JSON converter, or imported a crate for it.
    Right now `get` returns a BTreeMap preparing for that conversion.


    What happens if a struct  has other structs embedded in it?
    CALEB: I have included the way I would handle different structs, commented out.

    I was thinking, what if you kept track of the fields of the struct that you did set,
    rather than the ones that you did not.
    When you set Table_Id, make a note that you just set it.

    Instead of excluding what you didn't set,
    only include what you did set,

    Along those lines, just make a hashmap as you decode with shortbit,
    store everything in the hashmap, if it's not set, it won't be in the hashmap,
    you can avoid kv_cleaning altogether.
    CALEB: I could explore that. I tried a version of that, which you can see in `info.rs`, in a previous
    commit, though it's not exactly the same as you've described here.


    Once you have your hash map, marshal the data into the SpliceInfoSection,
    the keys of the hasmap are the fields that are set and should be in the JSON.
    Thst's what I do with xml, parse the data into hash map, and then marshal that into a SpliceInfoSection instance.

    Does that make sense to you?

    **/

    /// returns json struct, removing bad values
    fn kv_clean(&self) -> BTreeMap<String, CleanJson> {
        fn rec_clean(sit: SIType) -> CleanJson {
            match sit {
                SIType::Int(i) => CleanJson::Number(Number::Int(i)),
                SIType::Hex(h) => CleanJson::String(h),
                SIType::Flag(f) => CleanJson::Bool(f),
                SIType::Bytes(b) => CleanJson::Array(
                    b.iter()
                        .map(|u| CleanJson::Number(Number::Byte(*u)))
                        .collect(),
                ),
                SIType::SapType(st) => CleanJson::String(st),
                SIType::TimeStamp(ts) => CleanJson::Number(Number::Float(ts)),
                SIType::Map(hs) => CleanJson::Object(
                    hs.iter()
                        .map(|(k, v)| (k.to_owned(), rec_clean(v.clone())))
                        .collect(),
                ),
                SIType::OtherArray(oa) => {
                    CleanJson::Array(oa.iter().map(|v| rec_clean(v.clone())).collect())
                }
                SIType::SpliceInfoSection(sis) => CleanJson::Object(sis.get()),
                SIType::TimeSignal(ts) => CleanJson::Object(ts.get()),
                // ...
                _ => CleanJson::Null,
            }
        }
        self.iter()
            .filter_map(|(k, v)| {
                let val = rec_clean(v.clone());
                if let CleanJson::Null = val {
                    None
                } else {
                    Some((k.to_owned(), val))
                }
            })
            .collect()
    }

    /// returns instance as a `kv_clean`ed hashmap
    pub fn get(&self) -> BTreeMap<String, CleanJson> {
        self.kv_clean()
    }

    /// get sap details from `SAPS`
    fn get_sap_details(&self, sap_type: &SIType) -> SIType {
        let st = match sap_type {
            SIType::Hex(h) => h,
            _ => &String::new(),
        };
        let hex = st.trim_start_matches("0x").to_string();
        let idx = usize::from_str_radix(&hex, 16).expect("Could not convert hex value to usize");
        SIType::SapType(SAPS[idx].to_string())
    }

    /// get pts adjustment with `self.as_90k`
    fn get_pts(&self, ticks: &SbType) -> SIType {
        let t = match ticks {
            SbType::Int(i) => i,
            _ => &BigInt::ZERO,
        };
        let pts_adjustment_ticks = t.to_u64_digits().1[0];
        SIType::TimeStamp(self.as_90k(pts_adjustment_ticks))
    }

    /// converts `SbType` enums into `SIType`
    fn map_sb_si(&self, v: &SbType) -> SIType {
        match v {
            SbType::Hex(h) => SIType::Hex(h.clone()),
            SbType::Int(i) => SIType::Int(i.to_u64_digits().1[0]),
            SbType::Flag(f) => SIType::Flag(f.clone()),
            SbType::Bytes(b) => SIType::Bytes(b.clone()),
            _ => SIType::None,
        }
    }

    /// InfoSection.decode
    pub fn decode(&mut self, bites: &[u8]) {
        let mut shortb = ShortBit::new(bites);
        self.table_id = self.map_sb_si(&shortb.as_hex(8));
        self.section_syntax_indicator = self.map_sb_si(&shortb.as_flag(None));
        self.private = self.map_sb_si(&shortb.as_flag(None));
        self.sap_type = self.map_sb_si(&shortb.as_hex(2));
        self.sap_details = self.get_sap_details(&self.sap_type);
        self.section_length = self.map_sb_si(&shortb.as_int(12));
        self.protocol_version = self.map_sb_si(&shortb.as_int(8));
        self.encrypted_packet = self.map_sb_si(&shortb.as_flag(None));
        self.encryption_algorithm = self.map_sb_si(&shortb.as_int(6));
        self.pts_adjustment = self.get_pts(&shortb.as_int(33));
        self.cw_index = self.map_sb_si(&shortb.as_hex(8));
        self.tier = self.map_sb_si(&shortb.as_hex(12));
        self.splice_command_length = self.map_sb_si(&shortb.as_int(12));
        self.splice_command_type = self.map_sb_si(&shortb.as_int(8));
    }
}

struct Iter<'a> {
    inner: &'a SpliceInfoSection,
    index: u8,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a SIType);
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
