//! src/timesignal.rs

use std::collections::BTreeMap;

use crate::{
    json::{self, CleanJson, Number},
    short::ShortBit,
};

/// Table 11 - time_signal()
#[derive(Clone, Debug)]
pub struct TimeSignal {
    command_length: CleanJson,
    command_type: CleanJson,
    name: CleanJson,
    time_specified_flag: CleanJson,
    pts_time: CleanJson,
}

impl TimeSignal {
    pub fn new() -> TimeSignal {
        Self {
            command_length: CleanJson::Number(Number::Int(0)),
            command_type: CleanJson::Number(Number::Int(6)),
            name: CleanJson::String(String::from("Time Signal")),
            time_specified_flag: CleanJson::Null,
            pts_time: CleanJson::Null,
        }
    }

    /// ticks to 90k timestamps
    fn as_90k(&self, int_time: u64) -> f64 {
        let sig_figs = 6;
        let p = u64::pow(10, sig_figs);
        ((int_time * p) as f64 / 90000.0).round() / p as f64
    }

    /// sets TimeSignal.command_length
    fn set_len(&mut self, start: usize, end: usize) {
        self.command_length = CleanJson::Number(Number::Int((start - end) as u64 >> 3));
    }

    /// _splice_time Table 14 - splice_time()
    fn splice_time(&mut self, shortb: &mut ShortBit) {
        let time_specified_flag = shortb.as_flag();
        self.time_specified_flag = CleanJson::Bool(time_specified_flag.clone());
        if time_specified_flag {
            shortb.forward(6);
            self.pts_time = CleanJson::Number(Number::Float({
                let bs = shortb.as_int(33).to_u64_digits().1;
                self.as_90k(if bs.len() > 0 { bs[0] } else { 0 })
            }));
        } else {
            shortb.forward(7);
        }
    }

    /// TimeSognal.decode method
    pub fn decode(&mut self, bytes: &[u8]) {
        let mut shortb = ShortBit::new(bytes);
        let start = shortb.get_idx();
        self.splice_time(&mut shortb);
        self.set_len(start, shortb.get_idx());
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
        json::to_json(self.get())
    }

    /// prints self as formated json to stderr
    pub fn show(&self) {
        eprintln!("{}", json::to_pretty(self.json(), 4));
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
    inner: &'a TimeSignal,
    index: u8,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a str, &'a CleanJson);
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
