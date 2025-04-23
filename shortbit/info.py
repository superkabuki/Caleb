"""
Implement this in Rust
This will decode the first part of a SCTE-35 Cue. 
Depends on ShortBit
I  am curious to see what you do with kv_clean

"""
from short import ShortBit


sap_map = {
    "0x00": "Type 1 Closed GOP with no leading pictures",
    "0x01": "Type 2 Closed GOP with leading pictures",
    "0x02": "Type 3 Open GOP",
    "0x03": "No Sap Type",
}


class SpliceInfoSection:
    """
    The SCTE-35 splice info section
    data.
    """

    def __init__(self):
        self.table_id = None
        self.section_syntax_indicator = None
        self.private = None
        # sap_type used to be marked reserved.
        self.sap_type = None
        self.sap_details = None
        self.section_length = None
        self.protocol_version = None
        self.encrypted_packet = None
        self.encryption_algorithm = None
        self.pts_adjustment = 0
        self.cw_index = None
        self.tier = None
        self.splice_command_length = None
        self.splice_command_type = None
        self.descriptor_loop_length = 0
        self.crc = None

    @staticmethod
    def as_90k(int_time):
        """
        ticks to 90k timestamps
        """
        return round((int_time / 90000.0), 6)

    def kv_clean(self):
        """
        kv_clean recursively removes items
        from a dict if the value is None.
        """

        def b2l(val):
            if isinstance(val, SpliceInfoSection):
                val.kv_clean()
            if isinstance(val, (list)):
                val = [b2l(v) for v in val]
            if isinstance(val, (dict)):
                val = {k: b2l(v) for k, v in val.items()}
            if isinstance(val, (bytes, bytearray)):
                val = list(val)
            return val

        return {
            k: b2l(v)
            for k, v in vars(self).items()
            if v
            not in [
                None,
                [],
            ]
        }  

    def get(self):
        """
        Returns instance as a kv_clean'ed dict
        """
        return self.kv_clean()

    def decode(self, bites):
        """
        InfoSection.decode
        """
        shortb = ShortBit(bites) 
        self.table_id = shortb.as_hex(8)
        if self.table_id != "0xfc":
            red(f"splice_info_section.table_id should be 0xfc Not:  {self.table_id}")
        self.section_syntax_indicator = shortb.as_flag(1)
        self.private = shortb.as_flag(1)
        self.sap_type = shortb.as_hex(2)
        self.sap_details = sap_map[self.sap_type]
        self.section_length = shortb.as_int(12)
        self.protocol_version = shortb.as_int(8)
        self.encrypted_packet = shortb.as_flag(1)
        self.encryption_algorithm = shortb.as_int(6)
        pts_adjustment_ticks = shortb.as_int(33)
        self.pts_adjustment = self.as_90k(pts_adjustment_ticks) 
        self.cw_index = shortb.as_hex(8)
        self.tier = shortb.as_hex(12)
        self.splice_command_length = shortb.as_int(12)
        self.splice_command_type = shortb.as_int(8)
