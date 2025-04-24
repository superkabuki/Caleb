
__Null/ Nil values__

Here's a simplified example to demonstrate what I mean, maybe there is a better way tp handle it in rust.

__The problem is how to omit fields from a struct if they have not been set.__

```py3
>>>> from threefive import TimeSignal


>>>> ts=TimeSignal()
>>>> ts.time_specified_flag=False
>>>> ts.encode()
>>>> ts.show()
{
    "command_length": 0,
    "command_type": 6,
    "name": "Time Signal",   
    "time_specified_flag": false        # <---    No pts_time
}

>>>> ts2=TimeSignal()
>>>> ts2.time_specified_flag=True
>>>> ts2.pts_time=1234.567890
>>>> ts2.encode()
>>>> ts.show()
{
    "command_length": 0,
    "command_type": 6,
    "name": "Time Signal",
    "time_specified_flag": true,    # <--- time_specified_flag is set
    "pts_time": 1234.56789          # <--- so pts_time is included
}
```


  
  

<pre>
  
Convert the ShortBit class to rust.
look at short.py and short.go  and mimick the functionality. 
ShortBit takes a string of bytes, converts it to an arbitrarily large integer.
it has methods to slice of bits and return them as_int, as_hex etc..
These methods all take the number of bits to slice off as an arg.

  1) pass in bytes
  2) convert bytes to int
  3) create methods/functions to slice of bits
  and return them as either a booleaan, bytes,hex int, or string.
  
</pre>


* This is how it works.



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
