//! src/info2.rs

//#![allow(unused)]

use num_bigint::BigInt;

use crate::short;

/// This is not great but for now it works
static SAPS: [&'static str; 4] = [
    "Type 1 Closed GOP with no leading pictures",
    "Type 2 Closed GOP with leading pictures",
    "Type 3 Open GOP",
    "No Sap Type",
];

/// The SCTE-35 splice info section
pub struct SpliceInfoSection {
    table_id: Option<String>,               // hex
    section_syntax_indicator: Option<bool>, // flag
    private: Option<bool>,                  // flag
    sap_type: Option<String>,               // hex
    sap_details: Option<String>,            // string (saps)
    section_length: Option<BigInt>,         // int
    protocol_version: Option<BigInt>,       // int
    encrypted_packet: Option<bool>,         // flag
    encryption_algorithm: Option<BigInt>,   // int
    pts_adjustment: Option<f64>,            // f64
    cw_index: Option<String>,               // hex
    tier: Option<String>,                   // hex
    splice_command_length: Option<BigInt>,  // int
    splice_command_type: Option<BigInt>,    // int
    descriptor_loop_length: Option<BigInt>, // int
    crc: Option<String>,                    // hex
}

impl SpliceInfoSection {
    pub fn new() -> SpliceInfoSection {
        Self {
            table_id: None,
            section_syntax_indicator: None,
            private: None,
            sap_type: None,
            sap_details: None,
            section_length: None,
            protocol_version: None,
            encrypted_packet: None,
            encryption_algorithm: None,
            pts_adjustment: None,
            cw_index: None,
            tier: None,
            splice_command_length: None,
            splice_command_type: None,
            descriptor_loop_length: None,
            crc: None,
        }
    }

    /// ticks to 90k timestamps
    fn as_90k(&self, int_time: u64) -> f64 {
        let sig_figs = 6;
        let p = u64::pow(10, sig_figs);
        ((int_time * p) as f64 / 90000.0).round() / p as f64
    }

    /// removes items if the value is 'None'. Returns a hashmap.
    fn kv_clean(&self) {
        // rust does not natively give a way to loop through struct properties.
        // There is a way with serde, but I did not implement it.
        // The other version of `info` implements SpliceInfoSection with a hashmap
        
        // Here's one way
        // https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=66403bddbba1a79a3722eb9f4ed62099
     
        // here's about seven more
        // https://users.rust-lang.org/t/how-to-iterate-over-fields-of-struct/53356/5
    
        // Did you not see what I wrote on the README?
        // It doesn't matter how you exclude them, but JSON doesn't do Nil values.
    }

    /// returns instance as a `kv_clean`ed hashmap
    pub fn get(&self) {}

    /// get sap details from `SAPS`
    fn get_sap_details(&self, sap_type: Option<String>) -> Option<String> {
        let hex = sap_type
            .unwrap_or("03".to_string())
            .trim_start_matches("0x")
            .to_string();
        let idx = usize::from_str_radix(&hex, 16).expect("Could not convert hex value to usize");
        Some(SAPS[idx].to_string())
    }

    /// get pts adjustment with `self.as_90k`
    fn get_pts_adjustment(&self, ticks: Option<BigInt>) -> Option<f64> {
        let pts_adjustment_ticks = ticks.unwrap_or(BigInt::ZERO).to_u64_digits().1[0];
        Some(self.as_90k(pts_adjustment_ticks))
    }

    /// InfoSection.decode
    pub fn decode(&mut self, bites: &[u8]) {
        let mut shortb = short::ShortBit::new(bites);
        self.table_id = shortb.as_hex(8);
        self.section_syntax_indicator = shortb.as_flag(Some(1));
        self.private = shortb.as_flag(Some(1));
        self.sap_type = shortb.as_hex(2);
        self.sap_details = self.get_sap_details(self.sap_type.clone());
        self.section_length = shortb.as_int(12);
        self.protocol_version = shortb.as_int(8);
        self.encrypted_packet = shortb.as_flag(Some(1));
        self.encryption_algorithm = shortb.as_int(6);
        self.pts_adjustment = self.get_pts_adjustment(shortb.as_int(33));
        self.cw_index = shortb.as_hex(8);
        self.tier = shortb.as_hex(12);
        self.splice_command_length = shortb.as_int(12);
        self.splice_command_type = shortb.as_int(8);
    }
}
