# seen_set

A HashSet that doesn't store values, but instead only stores hashes.

Due to its nature, this is unsafe because it doesn't handle hash collisions, but
it maybe by useful in scenarios where that's tolerable (ex. some caching
scenarios).

This is useful when you only need to tell if you've seen a value before and you
don't want to clone the value for performance reasons.

```rs
let mut seen = SeenSet::new();
for path in some_func_that_may_return_duplicate_paths() {
  if !seen.insert(&path) {
    continue;
  }
  // we haven't seen this path before
}
```

Note: This will be slower for values like numbers. Just use a regular HashSet
for that.
