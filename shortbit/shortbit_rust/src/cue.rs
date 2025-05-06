//! src/cue.rs

use crate::{info2::SpliceInfoSection, timesignal::TimeSignal};

#[derive(Debug)]
pub struct Cue {
    pub info_section: SpliceInfoSection,
    pub time_signal: TimeSignal,
    pub bytes: Vec<u8>,
}

impl Cue {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            info_section: SpliceInfoSection::new(),
            time_signal: TimeSignal::new(),
            bytes,
        }
    }

    pub fn decode(&mut self) {
        let info_bytes = self.bytes[..14].to_vec();
        let time_signal_bytes = self.bytes[14..].to_vec();
        self.info_section.decode(&info_bytes);
        self.time_signal.decode(&time_signal_bytes);
    }
}
