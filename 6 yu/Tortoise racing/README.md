## Detail

[Tortoise racing](https://www.codewars.com/kata/tortoise-racing/train/rust)

Two tortoises named **A** and **B** must run a race. **A**starts with an average speed of `720 feet per hour`. Young **B** knows she runs faster than **A**, and furthermore has not finished her cabbage.

When she starts, at last, she can see that **A** has a `70 feet lead` but **B**'s speed is `850 feet per hour`. How long will it take **B** to catch **A**?

More generally: given two speeds `v1` (**A**'s speed, integer > 0) and `v2` (**B**'s speed, integer > 0) and a lead `g` (integer > 0) how long will it take **B** to catch **A**? 

The result will be an array `[hour, min, sec]`which is the time needed in hours, minutes and seconds (don't worry for fractions of second).

If `v1 >= v2` then return `nil`, `nothing`, `null`, `None` or `{-1, -1, -1}` for C++, C, Go, Nim.

/# Examples

```rust
race(720, 850, 70) => [0, 32, 18]
race(80, 91, 37)   => [3, 21, 49]
```

Note: you can see some other examples in "Your test cases".

## Thinking

Since *detail*.

Therefore:

```rust
v1 * time + lead = v2 * time
lead = v2 * time - v1 * time
lead = time * (v2 - v1)
time = lead / (v2 - v1)
```

