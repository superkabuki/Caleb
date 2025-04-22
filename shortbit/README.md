Convert the ShortBit class to rust.
ShortBit takes a string of bytes, converts it to arbitrarily large integer.
it has methods to slice of bits and return them as_int, as_hex etc..
These methods all take the number of bits to slice off as an arg.

This is it works.
```py3
>>>> from short import ShortBit
>>>> sb =ShortBit(b"lefthandofGod")

>>>> sb.as_int(8)# < -- slice of the  "l" in lefthandofGod
108       
sb.as_flag()   # return the 8th bit of the "e" as a boolean
False               #  "e" is ascii char code is 101 or 01100101. The 8th bit is a zero or False.
>>>> sb.as_int(7) # the last seven bits of the "e"
101                    
sb.as_bytes(32) # 32 bits gives us 4 bytes or b"ftha"
b'ftha'
```
