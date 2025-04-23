//! src/info.rs

use num_bigint::BigInt;

use crate::short;

/// The SCTE-35 splice info section
struct SpliceInfoSection {
    table_id: Option<String>,
    section_syntax_indicator: Option<String>,
    private: Option<bool>,
    sap_type: Option<String>,
    sap_details: Option<String>,
    section_length: Option<BigInt>,
    protocol_version: Option<BigInt>,
    encrypted_packet: Option<bool>,
    encryption_algorithm: Option<BigInt>,
    pts_adjustment: Option<BigInt>,
    cw_index: Option<String>,
    tier: Option<String>,
    splice_command_length: Option<BigInt>,
    descriptor_loop_length: Option<BigInt>,
    crc: Option<Option<BigInt>>,
}

impl SpliceInfoSection {
    fn new() {}

    fn as_90k(&self) {}

    fn kv_clean(&self) {}

    fn get(&self) {}

    fn decode(&mut self, bites: &[u8]) {
        let shortb = short::ShortBit::new(bites);
    }
}
