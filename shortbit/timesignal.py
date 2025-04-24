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

    def _splice_time(self, bitbin):
        """
        _splice_time Table 14 - splice_time()
        """
        self.time_specified_flag = bitbin.as_flag(1)
        if self.time_specified_flag:                             
            bitbin.forward(6)
            pts_time_ticks = bitbin.as_int(33)     # This is only included in the output if time_specified_flag is set
            self.pts_time = self.as_90k(pts_time_ticks)  
        else:
            bitbin.forward(7)

      def kv_clean(self):
        """
        kv_clean recursively removes items
        from a dict if the value is None.
        """

        def b2l(val):
            if isinstance(val, SCTE35Base):
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
        }  # added empty list []
  
    def decode(self):
        """
        TimeSignal.decode method
        """
        bitbin = Bitn(self.bites)
        start = bitbin.idx
        self._splice_time(bitbin)
        self._set_len(start, bitbin.idx)

    def get(self):
        """
        Returns instance as a kv_clean'ed dict
        """
        return self.kv_clean()

     def json(self):
        """
        json returns self as kv_clean'ed json
        """
        return json.dumps(self.get(), indent=4)

    def show(self):
        """
        show prints self as json to stderr (2)
        """
        print2(self.json())
