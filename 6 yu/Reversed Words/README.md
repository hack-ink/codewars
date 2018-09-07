## Detail

[Reversed Words](https://www.codewars.com/kata/51c8991dee245d7ddf00000e)

Complete the solution so that it reverses all of the words within the string passed in. 

Example:

```rust
reverse_words("The greatest victory is that which requires no battle")
// should return "battle no requires which that is victory greatest The"
```

## Thinking

`split_whitespace()`, `rev()`, `collect()` then `join()`!