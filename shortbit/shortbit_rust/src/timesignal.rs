//! src/timesignal.rs

/// Table 11 - time_signal()
pub struct TimeSignal {
    command_length: TSType,
    command_type: TSType,
    name: TSType,
    time_specified_flag: TSType,
    pts_time: TSType,
}

pub enum TSType {}

impl TimeSignal {
    pub fn new() {}
    /// sets TimeSignal.command_length
    pub fn set_len(&self) {}
    /// _splice_time Table 14 - splice_time()
    fn splice_time(&self) {}
    /// recursively removes items from a hashmap if the value is None
    fn kv_clean(&self) {}
    /// TimeSognal.decode method
    pub fn decode(&self) {}
    /// returns instance as a `kv_clean`ed hashmap
    pub fn get(&self) {}
    /// returns self as `kv_clean`ed json
    pub fn json(&self) {}
    /// show prints self as json to stderr
    pub fn show(&self) {}
    /// iterate through struct fields
    fn iter(&self) {}
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
            _ => return None,
        };
        self.index += 1;
        Some(ret)
    }
}
