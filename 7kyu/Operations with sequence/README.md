## Detail

[Operations with sequence](https://www.codewars.com/kata/596ddaccdd42c1cf0e00005c**Steps**

1. Square the numbers that are greater than zero.
2. Multiply by 3 every third number.
3. Multiply by -1 every fifth number.
4. Return the sum of the sequence.

**Example**
`{ -2, -1, 0, 1, 2 }` returns `-6`

```rust
1. { -2, -1, 0, 1 * 1, 2 * 2 }
2. { -2, -1, 0 * 3, 1, 4 }
3. { -2, -1, 0, 1, -1 * 4 }
4. -6
```

P.S.: The sequence consists only of integers. And try not to use "for", "while" or "loop" statements.)

## Thinking

Use `iter().enumerate()`, the index will help you a lot.