## Detail

[Find n'th Digit of a Number](https://www.codewars.com/kata/find-nth-digit-of-a-number/train/rust)

The function `findDigit` takes two numbers as input, `num` and `nth`. It outputs the nth digit of num (counting from right to left).

\#Note

-   If `num` is negative, ignore its sign and treat it as a positive value.
-   If `nth` is not positive, return -1.
-   Keep in mind that 42 = 00042. This means that `findDigit(42, 5)` would return 0.

\#Examples

```rust
findDigit(5673, 4)     returns 5
findDigit(129, 2)      returns 2
findDigit(-2825, 3)    returns 8
findDigit(-456, 4)     returns 0
findDigit(0, 20)       returns 0
findDigit(65, 0)       returns -1
findDigit(24, -8)      returns -1
```

## Thinking

There's trick: you can return 0 if you can't `get()` the `nth`.