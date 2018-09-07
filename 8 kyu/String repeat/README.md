## Detail

[String repeat](https://www.codewars.com/kata/57a0e5c372292dd76d000d7e)

Write a function called `repeatStr` which repeats the given string `string` exactly `n` times.

```rust
repeatStr(6, "I") // "IIIIII"
repeatStr(5, "Hello") // "HelloHelloHelloHelloHello"
```
## Thinking

Use `std::iter::repeat()` and `take()`.