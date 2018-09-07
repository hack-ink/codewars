## Detail

[Summy](https://www.codewars.com/kata/summy/train/rust)

Write a function that takes a string which has integers inside it separated by spaces, and your task is to convert each integer in the string into an integer and return their sum.

\# Example

```rust
summy("1 2 3") -> 6
```

Good luck!

## Thinking

Use `split_whitespace()`, `parse::<i32>()` and `sum()`.