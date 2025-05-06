# Notes on stuff

Tuesday 12:30am 
### cue.rs, that's what I wanted to see. Nice job.

* you could kv clean this way:
     * dump a struct to a map 
     * remove the nulls
     * turn map into json
 
 * Even better
 * https://crates.io/crates/bevy_reflect



![image](https://github.com/user-attachments/assets/b8b5e110-9013-40df-af88-12c97f3c65ac)

---------------------------

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

CALEB: The enum system I only implemented to more easily handle different possible types when iterating over struct fields.
For this purpose alone it was useful, but I agree, it's far from ideal. It
was a part of my initial solution that I didn't remove, it just stuck around. Looking at it now, the enum
for ShortBit doesn't even need to exist at all.

Adrian: That was the only thing I could think of , that you wanted to make an array and needed one type.
Don't be afraid to abandon old ideas, I have a bad habit of do that myself.  


I did a SCTE-35 as a service, it's kind of cool.
https://github.com/superkabuki/SCTE-35_threefive3/blob/main/sassy.md

CALEB: Yeah, I saw that, pretty cool.

But really, we should put this on hold and get back to the plan. 
You need to contribute to some repos. Doesn't matter what language, 
like I said, JavaScript jobs are plentyful and easy to get.

CALEB: I have some features I want to add on one of my existing projects, in JS. I'll start with that.

-------------------------------------------

Sunday 05/04/2025 2:00pm


Overall, you seem to be writing a lot more than you need, even without the JSON, you have a lot of extra code. 
Time Signal should half the size that it is, Why are you doing TsType and SBType? 
You're making your code harder to follow because you're going sideways. 
It's like when see someone driving and switching lanes a lot, 
they are moving sideways a lot, but not going any faster. 

CALEB: Yeah, a unique enum for each struct is not good, and neither is converting one to another. If I decide to keep the enums
around, I would have a global enum used by all structs and handle embeded structs separately. If kv_clean is not needed, or
if the cleaning is done via some other method not relying on struct iteration, I'll remove the enum system entirely.

Adrian: People are going to disect your code, expect a lot of critism, just learn to take it. A lot of times it's hard to hear, but worth hearing. It's hard to always see your own mistakes.


You want your code short but readable. 
Be lazy and only do what you have to do.
You should be able to do what you're doing with half as TimeSignal code.


__The Worst thing a programmer can be is clever.__  Be obvious and predictable.

Why do your own JSON?
Only write something like that if there is a reason.

CALEB: I made my own mostly just because I wanted to try it (it didn't take too long), and to limit the amount of external dependencies,
but that doesn't matter too much I guess. But you're right, no need to reinvent the wheel in this case.

What about `#[serde(skip_serializing_if="Option::is_none")]`

* https://github.com/serde-rs/json/issues/513

* https://stackoverflow.com/questions/53900612/how-do-i-avoid-generating-json-when-serializing-a-value-that-is-null-or-a-defaul

CALEB: I already knew serde was an option, I just chose not to use it. Probably should have.

iterate fields

* https://github.com/ChayimFriedman2/fields-iter
  * You don't have to iterate over the fields

CALEB: Same thing, avoiding dependencies. Probably should use it though.
I'm used to the JS world, where someone can build a node project and not know what's going on because they've installed 5000 npm packages
instead of writing their own stuff, so when something breaks, they don't know if it's their code or one of the packages at fault.

---------------
