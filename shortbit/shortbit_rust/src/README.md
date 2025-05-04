Why do your own JSON?
Only write something like that if there is a reason.

What about `#[serde(skip_serializing_if="Option::is_none")]`

* https://github.com/serde-rs/json/issues/513

* https://stackoverflow.com/questions/53900612/how-do-i-avoid-generating-json-when-serializing-a-value-that-is-null-or-a-defaul
