# Notes on stuff

Monday 05/25/2025 6:00pm

So earlier I was trying threefive3 and not using kv_clean to remove the None/null values. 
There were a lot of null values in some of the JSON, I found it distracting , but I guess
it really doesn't matter, all the code still works. I guess it's really just preference. 

----------------------------

Monday 05/05/2025 1:00am 

I really don't get the point of all the enum types, one of the points of doing shortBit that way it is 
so that you return the values in the proper types, but you're having to convert to and from in both directions, 
all of these fields can be handled with basic types, they don't need anything special.
Explain it to me what you're doing with all the enum types, I just don't get it.

I did a SCTE-35 as a service, it's kind of cool.
https://github.com/superkabuki/SCTE-35_threefive3/blob/main/sassy.md

But really, we should put this on hold and get back to the plan. 
You need to contribute to some repos. Doesn't matter what language, 
like I said, JavaScript jobs are plentyful and easy to get.

-------------------------------------------

Sunday 05/04/2025 2:00pm


Overall, you seem to be writing a lot more than you need, even without the JSON, you have a lot of extra code. 
Time Signal should half the size that it is, Why are you doing TsType and SBType? 
You're making your code harder to follow because you're going sideways. 
It's like when see someone driving and switching lanes a lot, 
they are moving sideways a lot, but not going any faster. 

You want your code short but readable. 
Be lazy and only do what you have to do.
You should be able to do what you're doing with half as TimeSignal code.


__The Worst thing a programmer can be is clever.__  Be obvious and predictable.

Why do your own JSON?
Only write something like that if there is a reason.

What about `#[serde(skip_serializing_if="Option::is_none")]`

* https://github.com/serde-rs/json/issues/513

* https://stackoverflow.com/questions/53900612/how-do-i-avoid-generating-json-when-serializing-a-value-that-is-null-or-a-defaul


iterate fields

* https://github.com/ChayimFriedman2/fields-iter
  * You don't have to iterate over the fields

---------------
