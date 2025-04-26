//! src/timesignal.rs

/// Table 11 - time_signal()
pub struct TimeSignal {
    command_length: usize,
    command_type: usize,
    name: String,
    time_specified_flag: Option<String>,
    pts_time: Option<usize>,
}

impl TimeSignal {
    pub fn new() {}
    /// sets TimeSignal.command_length
    pub fn set_len(&self) {}
    /// _splice_time Table 14 - splice_time()
    pub fn splice_time(&self) {}
    /// recursively removes items from a hashmap if the value is None
    pub fn kv_clean(&self) {}
    /// TimeSognal.decode method
    pub fn decode(&self) {}
    /// returns instance as a `kv_clean`ed hashmap
    pub fn get(&self) {}
    /// returns self as `kv_clean`ed json
    pub fn json(&self) {}
    /// show prints self as json to stderr
    pub fn show(&self) {}
}
