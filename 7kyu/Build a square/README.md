## Detail

[Build a square](https://www.codewars.com/kata/build-a-square/train/rust)

I will give you an integer. Give me back a shape that is as long and wide as the integer. The integer will be a whole number between 0 and 50.

Example: Integer = 3; I expect a 3x3 square back just like below as a string.

Solution:

```rust
+++
+++
+++
```

## Thinking

Think about `std::iter::repeat()` or `std::slice::repeat()`.
