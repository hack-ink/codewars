## Detail

[Get Nth Even Number](https://www.codewars.com/kata/5933a1f8552bc2750a0000ed)

Return the Nth Even Number

```rust
nthEven(1) //=> 0, the first even number is 0
nthEven(3) //=> 4, the 3rd even number is 4 (0, 2, 4)

nthEven(100) //=> 198
nthEven(1298734) //=> 2597466
```

The input will not be 0.

Hint: Think Math

## Thinking

Equation: `x = 2 * (n - 1)`.