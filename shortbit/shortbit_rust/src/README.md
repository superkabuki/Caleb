Overall, you seem to be writing a lot more than you need, even without the JSON, you have a lot of extra code. 
Time Signal should half the size that it is, Why are you doing TsType and SBType? 
You're making your code harder to follow because you're going sideways. 
It's like when see someone driving and switching lanes a lot, 
they are moving sideways a lot, but not going any faster. 

You want your code short but readable. 
Be lazy and only do what you have to do.
You should be able to do what you're doing with half as TimeSignal code.

__The Worst thing a programmer can be is clever.__ 

Why do your own JSON?
Only write something like that if there is a reason.

What about `#[serde(skip_serializing_if="Option::is_none")]`

* https://github.com/serde-rs/json/issues/513

* https://stackoverflow.com/questions/53900612/how-do-i-avoid-generating-json-when-serializing-a-value-that-is-null-or-a-defaul


iterate fields

* https://github.com/ChayimFriedman2/fields-iter
  * You don't have to iterate over the fields
