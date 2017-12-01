## Detail

[Square(n) Sum](https://www.codewars.com/kata/515e271a311df0350d00000f)

Complete the `squareSum`/`square_sum`/`SquareSum` method so that it squares each number passed into it and then sums the results together. 

For example:

```rust
square_sum([1, 2, 2]) // should return 9
```

## Thinking

Use `fold(0, |t, i| t + i * i)`.