"""
Implement this in Rust

The Cue class has two classes embedded SpliceInfoSection and TimeSignal.
Don't worry about JSON , or anything else not in here.
Try to do this as closely to this as you can, as rust.
Keep your existing ShortBit.


Depends on ShortBit

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


    def decode(self, bites):
        """
        InfoSection.decode
        """
        shortb = ShortBit(bites) 
        self.table_id = shortb.as_hex(8)
        self.section_syntax_indicator = shortb.as_flag(1)
        self.private = shortb.as_flag(1)
        self.sap_type = shortb.as_hex(2)
        self.sap_details = sap_map[self.sap_type]
        self.section_length = shortb.as_int(12)
        self.protocol_version = shortb.as_int(8)
        self.encrypted_packet = shortb.as_flag(1)
        self.encryption_algorithm = shortb.as_int(6)
        self.pts_adjustment = shortb.as_int(33)
        self.cw_index = shortb.as_hex(8)
        self.tier = shortb.as_hex(12)
        self.splice_command_length = shortb.as_int(12)
        self.splice_command_type = shortb.as_int(8)


class TimeSignal:
    """
    Table 11 - time_signal()
    """

    def __init__(self, bites=None):
        self.command_length= 0
        self.command_type = 6
        self.name = "Time Signal"
        self.time_specified_flag = None
        self.pts_time = None

    def _set_len(self, start, end):
        """
        _set_len sets
        self.command_length
        """
        self.command_length = (start - end) >> 3

    def _splice_time(self, shortbit):
        """
        _splice_time Table 14 - splice_time()
        """
        self.time_specified_flag = shortbit.as_flag(1)
        if self.time_specified_flag:                             
            shortbit.forward(6)
            self.pts_time = shortbit.as_int(33)  
        else:
            shortbit.forward(7)

    def decode(self):
        """
        TimeSignal.decode method
        """
        shortbit = ShortBit(self.bites)
        start = shortbit.idx
        self._splice_time(shortbit)
        self._set_len(start, shortbit.idx)

class Cue:
    def __init__(self,bites):
        self.infosection =SpliceInfoSecton()
        self.timesignal = TimeSignal()
        self.bites = bites

    def decode():
        info_bites= self.bites[:14]
        timesignal_bites= self.bites[14:]  # Info Section is always the same size
        self.infosection.decode(info_bites)
        self.timesignal.decode(timesignal_bites)


if __name__ == '__main__':
list_o_bites = [b'\xfc0\x16\x00\x00\x00\x00\x00\x00\x00\xff\xf0\x05\x06\xff+\x9d\x90\xf8\x00\x00\xe7\xe0Y4'
b'\xfc0\x16\x00\x00\x00\x00\x00\x00\x00\xff\xf0\x05\x06\xff+\xa0P\x18\x00\x00\x99\xda\xeaw'
b'\xfc0\x16\x00\x00\x00\x00\x00\x00\x00\xff\xf0\x05\x06\xff+\xa3\x0f8\x00\x00\xba\xb7\t\x97'
b'\xfc0\x16\x00\x00\x00\x00\x00\x00\x00\xff\xf0\x05\x06\xff+\xdf{\xf8\x00\x00w\xb1T\xad'
b'\xfc0\x16\x00\x00\x00\x00\x00\x00\x00\xff\xf0\x05\x06\xff+\xe2;\x18\x00\x00\xafm\xda\xf3'
b'\xfc0\x16\x00\x00\x00\x00\x00\x00\x00\xff\xf0\x05\x06\xff+\xeaxx\x00\x00\x12\xdeN\xee']
for bites in list_o_bites:
    cue= Cue(bites)
    cue.decode()
    print(cue)

    
