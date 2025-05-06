# SCTE-35-Rust

Running `cue.decode()` on `b"\xfc0\x16\x00\x00\x00\x00\x00\x00\x00\xff\xf0\x05\x06\xff+\x9d\x90\xf8\x00\x00\xe7\xe0Y4"`

results in:

``` bash
{
    "bytes": [
        252,
         48,
         22,
         0,
         0,
         0,
         0,
         0,
         0,
         0,
         255,
         240,
         5,
         6,
         255,
         43,
         157,
         144,
         248,
         0,
         0,
         231,
         224,
         89,
         52
    ],
    "time_signal": {
        "command_length": 5,
        "command_type": 6,
        "pts_time": 55852.376089,
        "time_specified_flag": true
    },
    "info_section": {
        "cw_index": "0x00",
        "encrypted_packet": false,
        "encryption_algorithm": 0,
        "private": false,
        "protocol_cersion": 0,
        "pts_adjustment": 0,
        "sap_details": "No Sap Type",
        "sap_type": "0x03",
        "section_length": 22,
        "section_syntax_indicator": false,
        "splice_command_length": 5,
        "splice_command_type": 6,
        "table_id": "0xfc",
        "tier": "0x0fff"
    }
}
```
