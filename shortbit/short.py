
class ShortBit:
    """
    short.ShortBit takes a byte string and
    converts it to a integer, a very large integer
    if needed. A 1500 bit integer is no problem.
    several methods are available for slicing off bits.
    """

    def __init__(self, bites):
        self.bitsize = self.idx = len(bites) << 3
        self.bits = int.from_bytes(bites, byteorder="big")

    def as_int(self, num_bits):
        """
        Starting at self.idx of self.bits,
        slice off num_bits of bits.
        """
        if self.idx >= num_bits:
            self.idx -= num_bits
            return (self.bits >> (self.idx)) & ~(~0 << num_bits)
        return False

    def as_hex(self, num_bits):
        """
        Returns the hex value
        of num_bits of bits
        """
        hexed = hex(self.as_int(num_bits))
        return (hexed.replace("0x", "0x0", 1), hexed)[len(hexed) % 2 == 0] # This is python specific code
                                                                                                                    # just look at the and match it in rust.
    def as_bytes(self, num_bits):
        """
        Returns num_bits of bits
        as bytes
        """
        gonzo = self.as_int(num_bits)
        wide = num_bits >> 3
        return int.to_bytes(gonzo, wide, byteorder="big")

    def as_flag(self, num_bits=1):
        """
        Returns one bit as True or False
        """
        return self.as_int(num_bits) & 1 == 1

    def forward(self, num_bits):
        """
        Advances the start point
        forward by num_bits
        """
        self.idx -= num_bits


    def negative_shift(self, num_bits):
        """
        negative_shift is called instead of
        throwing a negative shift count error.
        """
        
        print(f"{num_bits} bits requested, but only {self.idx} bits left.")
        print(f"\n bytes remaining: {self.as_bytes(self.idx)} ")

