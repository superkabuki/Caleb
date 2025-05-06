//! src/timesignal.rs

use std::collections::BTreeMap;

use num_bigint::BigInt;

use crate::{
    json::{self, CleanJson, Number},
    short::{SbType, ShortBit},
};

#[derive(Clone, Debug)]
pub enum TSType {
    Int(u64),
    Hex(String),
    Flag(bool),
    Bytes(Vec<u8>),
    TimeStamp(f64),
    String(String),
    Map(BTreeMap<String, TSType>),
    OtherArray(Vec<TSType>),
    // SCTE35Base(Box<SCTE35Base>),
    // ...
    None,
}

/// Table 11 - time_signal()
#[derive(Clone, Debug)]
pub struct TimeSignal {
    command_length: TSType,
    command_type: TSType,
    name: TSType,
    time_specified_flag: TSType,
    pts_time: TSType,
}

impl TimeSignal {
    pub fn new() -> TimeSignal {
        Self {
            command_length: TSType::Int(0),
            command_type: TSType::Int(6),
            name: TSType::String(String::from("Time Signal")),
            time_specified_flag: TSType::None,
            pts_time: TSType::None,
        }
    }

    /// ticks to 90k timestamps
    fn as_90k(&self, int_time: u64) -> f64 {
        let sig_figs = 6;
        let p = u64::pow(10, sig_figs);
        ((int_time * p) as f64 / 90000.0).round() / p as f64
    }

    /// get pts adjustment with `self.as_90k`
    fn get_pts(&self, ticks: &SbType) -> TSType {
        let t = match ticks {
            SbType::Int(i) => i,
            _ => &BigInt::ZERO,
        };
        let pts_adjustment_ticks = t.to_u64_digits().1[0];
        TSType::TimeStamp(self.as_90k(pts_adjustment_ticks))
    }

    /// sets TimeSignal.command_length
    fn set_len(&mut self, start: usize, end: usize) {
        self.command_length = TSType::Int((start - end) as u64 >> 3);
    }

    /// _splice_time Table 14 - splice_time()
    fn splice_time(&mut self, shortb: &mut ShortBit) {
        self.time_specified_flag = self.map_sb_ts(&shortb.as_flag(None));
        if let TSType::Flag(f) = self.time_specified_flag {
            if f {
                shortb.forward(6);
                let pts_time_ticks = shortb.as_int(33);
                self.pts_time = self.get_pts(&pts_time_ticks);
            } else {
                shortb.forward(7);
            }
        }
    }

    /// recursively removes items from a map if the value is None
    fn kv_clean(&self) -> BTreeMap<String, CleanJson> {
        fn rec_clean(sit: TSType) -> CleanJson {
            match sit {
                TSType::Int(i) => CleanJson::Number(Number::Int(i)),
                TSType::Hex(h) => CleanJson::String(h),
                TSType::Flag(f) => CleanJson::Bool(f),
                TSType::Bytes(b) => CleanJson::Array(
                    b.iter()
                        .map(|u| CleanJson::Number(Number::Byte(*u)))
                        .collect(),
                ),
                TSType::TimeStamp(ts) => CleanJson::Number(Number::Float(ts)),
                TSType::Map(hs) => CleanJson::Object(
                    hs.iter()
                        .map(|(k, v)| (k.to_owned(), rec_clean(v.clone())))
                        .collect(),
                ),
                TSType::OtherArray(oa) => {
                    CleanJson::Array(oa.iter().map(|v| rec_clean(v.clone())).collect())
                }
                // TSType::SCTE35Base(s35b) => CleanJson::Object(s35b.get()),
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

    /// TimeSognal.decode method
    pub fn decode(&mut self, bytes: &[u8]) {
        let mut shortb = ShortBit::new(bytes);
        let start = shortb.get_idx();
        self.splice_time(&mut shortb);
        self.set_len(start, shortb.get_idx());
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

    /// iterate through struct fields
    fn iter(&self) -> Iter<'_> {
        Iter {
            inner: self,
            index: 0,
        }
    }

    /// converts `SbType` enums into `TSType`
    fn map_sb_ts(&self, v: &SbType) -> TSType {
        match v {
            SbType::Hex(h) => TSType::Hex(h.clone()),
            SbType::Int(i) => TSType::Int(i.to_u64_digits().1[0]),
            SbType::Flag(f) => TSType::Flag(f.clone()),
            SbType::Bytes(b) => TSType::Bytes(b.clone()),
            _ => TSType::None,
        }
    }
}

struct Iter<'a> {
    inner: &'a TimeSignal,
    index: u8,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a TSType);
    fn next(&mut self) -> Option<Self::Item> {
        let ret = match self.index {
            0 => ("command_length", &self.inner.command_length),
            1 => ("command_type", &self.inner.command_type),
            2 => ("name", &self.inner.name),
            3 => ("time_specified_flag", &self.inner.time_specified_flag),
            4 => ("pts_time", &self.inner.pts_time),
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}
