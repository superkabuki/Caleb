Tuesday 3:00 pm <br>
Adrian: here are my concerns with using the enums
* You lose the benefits of a strongly type language, mainly performance and memory allocation.It's not a big deal in this example, but when you call something 50 million times, it starts to add up.
* function calls, you'll have to convert from the enums to a standard type before you call any function. Like as_90k, you want to be abtle to share that function for both info section and timesignal.
* You also lose style points doing stuff like that, it's often frowned up in the strongly typed language circles.  


### profiling
* Look into profiling, it's the only way to really see what's happening. I used it Sunday to cut my runtime in half for a program.

* I was generating a table every time I did a crc32, if I do it once it's not even a blip 
```js
      256   -0.000   -0.000   -0.000   -0.000 crc.py:13(_bytecrc)  # this is called by _mk_table 256 times
        1   -0.000   -0.000   -0.000   -0.000 crc.py:21(_mk_table)
        1   -0.000   -0.000   -0.000   -0.000 crc.py:23(<listcomp>)
        1   -0.000   -0.000   -0.000   -0.000 crc.py:26(crc32)
```
* When I inject packets, I have to do a lot of crc32s and it was taking 39 seconds to run, so I profiled it and half of the time was spent on crc32s.
```js
187919717 function calls (187919680 primitive calls) in 39.337 seconds

 50171136   16.461    0.000   16.461    0.000 crc.py:13(_bytecrc)
   195981    0.085    0.000   17.279    0.000 crc.py:21(_mk_table)
   195981    0.732    0.000   17.194    0.000 crc.py:23(<listcomp>)
   195981    0.168    0.000   17.447    0.000 crc.py:26(crc32)
```
* I switched to a static table for the crc32s, and it dropped to 20 seconds.
```js
138614137 function calls (138614100 primitive calls) in 19.792 seconds

   195981    0.188    0.000    0.188    0.000 crc.py:11(crc32)
```




# SCTE-35-Rust

Running cue.decode() with what code?

Adrian: This is the cue.py stuff? Cool. 
Yeah man you're doing a good job, seriously.

CALEB: Look in `src/main.rs`. Run `cargo run` from the root of the rust project, if you have rust installed.
I didn't implement `json()` or `show()` for `Cue`, just did a quick version in `main.rs`.

The state of `Cue` after running `cue.decode()` on `b"\xfc0\x16\x00\x00\x00\x00\x00\x00\x00\xff\xf0\x05\x06\xff+\x9d\x90\xf8\x00\x00\xe7\xe0Y4"`

results in (converted to 'pretty' json for readability):

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
