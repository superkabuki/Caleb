//! src/info.rs

//#![allow(unused)]

use std::collections::HashMap;

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
///
/// stores everything in a single hashmap as strings.
/// This is not a suitable solution if the data is to
/// be changed after decoding and before exporting.
///
/// If change after decoding is required, the HashMap
/// could hold an enum of the data type as the value.
/// `kv_clean` could then convert those types into
/// strings only for exporting.
pub struct SpliceInfoSection {
    spliced_info: HashMap<String, String>,
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

    /// this version simply removes empty strings.
    fn kv_clean(&self) -> HashMap<String, String> {
        self.spliced_info
            .iter()
            .filter_map(|(k, v)| {
                if v.len() == 0 {
                    None
                } else {
                    Some((k.to_owned(), v.to_owned()))
                }
            })
            .collect()
    }

    /// returns instance as a `kv_clean`ed hashmap
    pub fn get(&self) -> HashMap<String, String> {
        self.kv_clean()
    }

    /// get sap details from `SAPS`
    fn get_sap(&self, sap_type: String) -> Option<String> {
        let hex = sap_type.trim_start_matches("0x").to_string();
        let idx = usize::from_str_radix(&hex, 16).expect("Could not convert hex value to usize");
        Some(SAPS[idx].to_string())
    }

    /// get pts adjustment with `self.as_90k`
    fn get_pts(&self, ticks: Option<BigInt>) -> Option<f64> {
        let pts_adjustment_ticks = ticks.unwrap_or(BigInt::ZERO).to_u64_digits().1[0];
        Some(self.as_90k(pts_adjustment_ticks))
    }

    /// InfoSection.decode
    pub fn decode(&mut self, bites: &[u8]) {
        let mut sb = short::ShortBit::new(bites);
        self.spliced_info.extend(
            [
                ("table_id", format!("{}", sb.as_hex(8).unwrap())),
                (
                    "section_syntax_indicator",
                    format!("{}", sb.as_flag(Some(1)).unwrap()),
                ),
                ("private", format!("{}", sb.as_flag(Some(1)).unwrap())),
            ]
            .map(|v| (v.0.to_string(), v.1)),
        );

        // sap type
        let sap_type = sb.as_hex(8).unwrap();

        self.spliced_info.extend(
            [
                ("sap_type", format!("{}", sap_type.clone())),
                (
                    "sap_details",
                    format!("{}", self.get_sap(sap_type).unwrap()),
                ),
            ]
            .map(|v| (v.0.to_string(), v.1)),
        );

        self.spliced_info.extend(
            [
                ("section_length", format!("{}", sb.as_int(12).unwrap())),
                ("protocol_version", format!("{}", sb.as_int(8).unwrap())),
                (
                    "encrypted_packet",
                    format!("{}", sb.as_flag(Some(1)).unwrap()),
                ),
                ("encryption_algorithm", format!("{}", sb.as_int(6).unwrap())),
                (
                    "pts_adjustment",
                    format!("{}", self.get_pts(sb.as_int(33)).unwrap()),
                ),
                ("cw_index", format!("{}", sb.as_hex(8).unwrap())),
                ("tier", format!("{}", sb.as_hex(8).unwrap())),
                (
                    "splice_command_length",
                    format!("{}", sb.as_int(8).unwrap()),
                ),
                ("splice_command_type", format!("{}", sb.as_int(8).unwrap())),
            ]
            .map(|v| (v.0.to_string(), v.1)),
        );
    }
}
