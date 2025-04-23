```js

$ ./353 sixfix ~/mpegts2/mpegts/3.ts
fixing these pids [258, 266, 260, 269]
Wrote: sixfixed-3.ts

         51820991 function calls in 160.138 seconds

   Ordered by: standard name

   ncalls  tottime  percall  cumtime  percall filename:lineno(function)
        1    0.000    0.000    0.000    0.000 353:408(dashdashhelp)
        1    0.000    0.000  160.138  160.138 353:421(chk_cli_map)
        1    0.000    0.000  160.138  160.138 353:436(go)
        // This is the bit stuff
      641    0.002    0.000    0.003    0.000 base.py:113(idxsplit)
     3201    0.010    0.000    0.021    0.000 base.py:63(as_90k)
    16987    0.026    0.000    0.026    0.000 bitn.py:104(__init__)
  1257038    3.683    0.000    5.371    0.000 bitn.py:109(nbits2bites)
   118909    0.336    0.000    0.495    0.000 bitn.py:120(add_bites)
  3499322    7.372    0.000   12.743    0.000 bitn.py:132(add_int)
    19577    0.092    0.000    0.150    0.000 bitn.py:16(__init__)
   628519    3.950    0.000    7.434    0.000 bitn.py:173(reserve)
  1412686    2.370    0.000    2.370    0.000 bitn.py:32(as_int)
    22147    0.165    0.000    0.300    0.000 bitn.py:42(as_hex)
        4    0.000    0.000    0.000    0.000 bitn.py:50(as_charset)
   118909    0.505    0.000    0.875    0.000 bitn.py:65(as_bytes)
     5398    0.015    0.000    0.024    0.000 bitn.py:74(as_flag)
   646860    0.907    0.000    0.907    0.000 bitn.py:80(forward)
// The bit stuff gets used a lot


// This the Splice Command code, it is super tight.
     1282    0.005    0.000    0.007    0.000 commands.py:121(__init__)
     1262    0.008    0.000    0.045    0.000 commands.py:128(decode)
     1264    0.010    0.000    0.026    0.000 commands.py:146(_splice_time)
     1282    0.002    0.000    0.002    0.000 commands.py:16(__init__)
       20    0.000    0.000    0.000    0.000 commands.py:195(__init__)
        8    0.000    0.000    0.000    0.000 commands.py:212(decode_break_duration)
       20    0.000    0.000    0.002    0.000 commands.py:221(decode)
     1282    0.002    0.000    0.002    0.000 commands.py:27(_set_len)

// You haven't seen the crc code, it's wacky. Clearly _bytecrc needs a tune up. 
  4348672   14.630    0.000   14.630    0.000 crc.py:11(_bytecrc)
    16987    0.068    0.000   21.023    0.001 crc.py:20(_mk_table)
    16987    6.326    0.000   20.956    0.001 crc.py:23(&lt;listcomp&gt;)
    16987    0.862    0.000   21.885    0.001 crc.py:26(crc32)
// Cue runs very well, despite me constantly trying to break it.

      641    0.002    0.000    0.004    0.000 cue.py:170(_pkt_bits)
      641    0.004    0.000    0.011    0.000 cue.py:178(_byte_bits)
      641    0.008    0.000    0.048    0.000 cue.py:183(_mk_bits)
     1282    0.005    0.000    0.014    0.000 cue.py:203(_mk_descriptors)
     1282    0.005    0.000    0.155    0.000 cue.py:217(mk_info_section)
     1282    0.008    0.000    0.062    0.000 cue.py:228(_set_splice_command)
      641    0.006    0.000    0.191    0.000 cue.py:45(__init__)
     1282    0.013    0.000    0.247    0.000 cue.py:63(decode)
     1282    0.004    0.000    0.009    0.000 cue.py:82(_descriptor_loop)
// descriptors is tuned up
       26    0.000    0.000    0.000    0.000 descriptors.py:28(__init__)
       26    0.000    0.000    0.000    0.000 descriptors.py:291(__init__)
       26    0.000    0.000    0.003    0.000 descriptors.py:317(decode)
       26    0.000    0.000    0.001    0.000 descriptors.py:330(_decode_flags)
       26    0.000    0.000    0.001    0.000 descriptors.py:342(_decode_segmentation)
       26    0.000    0.000    0.000    0.000 descriptors.py:355(_decode_segments)
       26    0.000    0.000    0.000    0.000 descriptors.py:37(parse_tag_and_len)
       26    0.000    0.000    0.000    0.000 descriptors.py:392(mk_the_upid)
       26    0.000    0.000    0.000    0.000 descriptors.py:48(parse_id)
       26    0.000    0.000    0.004    0.000 descriptors.py:511(splice_descriptor)
// You should do new_reader in rust. 
        2    0.000    0.000    0.000    0.000 new_reader.py:29(reader)
      641    0.001    0.000    0.001    0.000 packetdata.py:16(__init__)
      641    0.002    0.000    0.002    0.000 packetdata.py:22(mk_pcr)
      641    0.002    0.000    0.010    0.000 packetdata.py:32(mk_pts)
    16987    0.053    0.000    0.180    0.000 pmt.py:103(parse_descriptors)
    16987    0.922    0.000    8.275    0.000 pmt.py:116(parse_streams)
    16987    1.154    0.000   43.716    0.003 pmt.py:132(mk)
    16987    0.024    0.000    0.024    0.000 pmt.py:133(&lt;listcomp&gt;)
    16987    0.026    0.000    0.026    0.000 pmt.py:145(&lt;listcomp&gt;)
    16987    0.037    0.000    0.037    0.000 pmt.py:147(&lt;listcomp&gt;)
   118909    0.667    0.000    2.881    0.000 pmt.py:23(add)
   288779    2.887    0.000    6.980    0.000 pmt.py:30(__init__)
   288779    2.612    0.000   18.262    0.000 pmt.py:65(add)
    16987    0.513    0.000    9.740    0.001 pmt.py:77(__init__)
   118909    0.646    0.000    1.727    0.000 pmt.py:9(__init__)
      641    0.005    0.000    0.005    0.000 section.py:27(__init__)
     1282    0.033    0.000    0.150    0.000 section.py:47(decode)
    16987    0.052    0.000    0.181    0.000 sixfix.py:108(_chk_payload)
    16987    0.057    0.000    0.057    0.000 sixfix.py:112(_unpad_pmt)
    16987    0.111    0.000    0.177    0.000 sixfix.py:117(_mk_pmt_head)
    16987    0.161    0.000   44.121    0.003 sixfix.py:134(_process_pmt)
    16987    0.054    0.000   44.174    0.003 sixfix.py:146(pmt2packets)
    16987    0.103    0.000    0.363    0.000 sixfix.py:154(_pmt_precheck)
    16987    0.054    0.000    9.795    0.001 sixfix.py:164(mk_pmt)
    16987    0.219    0.000   54.748    0.003 sixfix.py:168(_parse_pmt)
      641    0.002    0.000    0.003    0.000 sixfix.py:18(passed)
        1    0.000    0.000    0.000    0.000 sixfix.py:207(_parse_program_streams)
       17    0.000    0.000    0.000    0.000 sixfix.py:224(_parse_stream_type)
        1    0.000    0.000  160.136  160.136 sixfix.py:240(sixfix)
        1    0.000    0.000  160.138  160.138 sixfix.py:259(cli)
        1    0.002    0.002  160.138  160.138 sixfix.py:261(&lt;listcomp&gt;)
        1    0.000    0.000   66.987   66.987 sixfix.py:32(decode)
        1    0.000    0.000    0.000    0.000 sixfix.py:49(__init__)
        1    0.000    0.000    0.000    0.000 sixfix.py:64(iter_pkts)
  4232482    6.184    0.000   61.181    0.000 sixfix.py:70(_parse_by_pid)
        1   19.256   19.256   92.819   92.819 sixfix.py:84(_parse_pkts)
        1    0.000    0.000   93.149   93.149 sixfix.py:99(convert_pids)
        2    0.000    0.000    0.000    0.000 stream.py:110(__init__)
        2    0.000    0.000    0.000    0.000 stream.py:140(__init__)
  4232482    5.746    0.000    5.746    0.000 stream.py:177(_pusi_flag)
    29400    0.040    0.000    0.040    0.000 stream.py:193(_pts_flag)
   340409    0.469    0.000    0.469    0.000 stream.py:198(_parse_length)
  8770750   12.160    0.000   12.160    0.000 stream.py:205(_parse_pid)
    33978    0.047    0.000    0.047    0.000 stream.py:213(_parse_program)
    34623    0.099    0.000    0.153    0.000 stream.py:220(_split_by_idx)
        1    0.000    0.000    0.000    0.000 stream.py:227(_find_start)
        1    0.000    0.000    0.000    0.000 stream.py:243(iter_pkts)
     3024    0.019    0.000   66.739    0.022 stream.py:251(_mk_pkts)
     3024    6.812    0.002   66.715    0.022 stream.py:252(&lt;listcomp&gt;)
     3024    0.017    0.000   66.840    0.022 stream.py:257(_decode2cues)
     3024    0.080    0.000    0.084    0.000 stream.py:258(&lt;listcomp&gt;)
        1    0.147    0.147   66.986   66.986 stream.py:260(decode)
    63374    0.102    0.000    0.102    0.000 stream.py:352(pid2prgm)
    15683    0.043    0.000    0.065    0.000 stream.py:386(_unpad)
      641    0.005    0.000    0.019    0.000 stream.py:404(_mk_packet_data)
    29400    0.229    0.000    0.569    0.000 stream.py:421(_parse_pts)
    87126    0.121    0.000    0.121    0.000 stream.py:449(_afc_flag)
    87126    0.285    0.000    0.471    0.000 stream.py:454(_parse_payload)
        2    0.000    0.000    0.000    0.000 stream.py:47(__init__)
    16987    0.081    0.000    5.078    0.000 stream.py:474(_pmt_pid)
        4    0.000    0.000    0.000    0.000 stream.py:482(_pat_pid)
        4    0.000    0.000    0.000    0.000 stream.py:486(_sdt_pid)
    57081    0.257    0.000    5.654    0.000 stream.py:490(_parse_tables)
  4232482   12.136    0.000   23.410    0.000 stream.py:504(_parse_info)
   187480    0.303    0.000    0.872    0.000 stream.py:518(_chk_pts)
  4232482   23.429    0.000   59.904    0.000 stream.py:528(_parse)
  4232482    6.077    0.000    6.077    0.000 stream.py:537(_pid_has_scte35)
    34623    0.105    0.000    0.258    0.000 stream.py:543(_chk_partial)
    57081    0.103    0.000    0.103    0.000 stream.py:548(_same_as_last)
    34623    0.122    0.000    0.190    0.000 stream.py:555(_section_incomplete)
      641    0.005    0.000    0.330    0.001 stream.py:563(_parse_cue)
      645    0.005    0.000    0.016    0.000 stream.py:571(_strip_scte35_pes)
      645    0.002    0.000    0.009    0.000 stream.py:584(_chk_maybe_pid)
      645    0.004    0.000    0.028    0.000 stream.py:593(_mk_scte35_payload)
      645    0.006    0.000    0.370    0.001 stream.py:599(_parse_scte35)
        2    0.000    0.000    0.000    0.000 stream.py:613(_mk_pinfo)
        2    0.000    0.000    0.000    0.000 stream.py:620(_parse_sdt)
        2    0.000    0.000    0.000    0.000 stream.py:654(_parse_pat)
    16987    0.246    0.000    4.972    0.000 stream.py:676(_parse_pmt)
    16987    0.564    0.000    4.417    0.000 stream.py:703(_parse_program_streams)
   288779    2.085    0.000    3.853    0.000 stream.py:718(_parse_stream_type)
   288813    0.500    0.000    0.589    0.000 stream.py:728(_set_scte35_pids)
        2    0.000    0.000    0.000    0.000 stream.py:90(__init__)
        1    0.000    0.000    0.000    0.000 stuff.py:16(print2)
     1282    0.010    0.000    0.022    0.000 stuff.py:46(clean)
      641    0.002    0.000    0.010    0.000 stuff.py:73(isjson)
      641    0.003    0.000    0.016    0.000 stuff.py:84(isxml)
       16    0.000    0.000    0.000    0.000 upids.py:100(decode)
        6    0.000    0.000    0.000    0.000 upids.py:118(decode)
       26    0.000    0.000    0.000    0.000 upids.py:22(__init__)
        4    0.000    0.000    0.000    0.000 upids.py:30(decode)
        1    0.000    0.000  160.138  160.138 {built-in method builtins.exec}
   312208    0.413    0.000    0.413    0.000 {built-in method builtins.hex}
   123398    0.165    0.000    0.165    0.000 {built-in method builtins.isinstance}
        2    0.000    0.000    0.000    0.000 {built-in method builtins.iter}
   161681    0.212    0.000    0.212    0.000 {built-in method builtins.len}
        2    0.000    0.000    0.000    0.000 {built-in method builtins.print}
     3201    0.010    0.000    0.010    0.000 {built-in method builtins.round}
    33974    0.047    0.000    0.047    0.000 {built-in method builtins.sum}
    20859    0.034    0.000    0.034    0.000 {built-in method from_bytes}
        3    0.139    0.046    0.139    0.046 {built-in method io.open}
        1    0.000    0.000    0.000    0.000 {built-in method sys.exit}
        1    0.191    0.191    0.191    0.191 {method &apos;__exit__&apos; of &apos;_io._IOBase&apos; objects}
    84944    0.111    0.000    0.111    0.000 {method &apos;add&apos; of &apos;set&apos; objects}
   493290    0.640    0.000    0.640    0.000 {method &apos;append&apos; of &apos;list&apos; objects}
     1312    0.006    0.000    0.006    0.000 {method &apos;decode&apos; of &apos;bytes&apos; objects}
        1    0.000    0.000    0.000    0.000 {method &apos;disable&apos; of &apos;_lsprof.Profiler&apos; objects}
     3242    0.006    0.000    0.006    0.000 {method &apos;getbuffer&apos; of &apos;_io.BytesIO&apos; objects}
    35264    0.055    0.000    0.055    0.000 {method &apos;index&apos; of &apos;bytes&apos; objects}
        1    0.000    0.000    0.000    0.000 {method &apos;items&apos; of &apos;dict&apos; objects}
        4    0.000    0.000    0.000    0.000 {method &apos;pop&apos; of &apos;dict&apos; objects}
        2    0.000    0.000    0.000    0.000 {method &apos;read&apos; of &apos;_io.BufferedReader&apos; objects}
        1    0.000    0.000    0.000    0.000 {method &apos;remove&apos; of &apos;list&apos; objects}
    22147    0.033    0.000    0.033    0.000 {method &apos;replace&apos; of &apos;str&apos; objects}
        2    0.000    0.000    0.000    0.000 {method &apos;rsplit&apos; of &apos;str&apos; objects}
      645    0.002    0.000    0.002    0.000 {method &apos;split&apos; of &apos;bytes&apos; objects}
      641    0.001    0.000    0.001    0.000 {method &apos;startswith&apos; of &apos;bytes&apos; objects}
        6    0.000    0.000    0.000    0.000 {method &apos;startswith&apos; of &apos;str&apos; objects}
    16965    0.023    0.000    0.023    0.000 {method &apos;strip&apos; of &apos;bytes&apos; objects}
  1375951    1.852    0.000    1.852    0.000 {method &apos;to_bytes&apos; of &apos;int&apos; objects}
     3242    0.900    0.000    0.900    0.000 {method &apos;write&apos; of &apos;_io.BufferedWriter&apos; objects}
  4215495    5.587    0.000    5.587    0.000 {method &apos;write&apos; of &apos;_io.BytesIO&apos; objects}
```
