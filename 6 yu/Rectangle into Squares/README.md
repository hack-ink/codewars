## Detail

[Rectangle into Squares](https://www.codewars.com/kata/rectangle-into-squares/train/rust)

The drawing below gives an idea of how to cut a given "true" rectangle into squares ("true" rectangle meaning that the two dimensions are different).

![alternative text](http://i.imgur.com/lk5vJ7sm.jpg)

Can you translate this drawing into an algorithm?

You will be given two dimensions 

1. a positive integer length (parameter named `lng`)
2. a positive integer width (parameter named `wdth`)

You will return an array with the size of each of the squares.

Shell bash returns a string.

```rust
  sqInRect(5, 3) should return [3, 2, 1, 1]
  sqInRect(3, 5) should return [3, 2, 1, 1]
```

\#Note: lng == wdth as a starting case would be an entirely different problem and the drawing is planned to be interpreted with lng != wdth. See kata, [Square into Squares. Protect trees!](http://www.codewars.com/kata/54eb33e5bc1a25440d000891).

When the initial parameters are so that `lng` == `wdth`, the solution `[lng]` would be the most obvious but not in the spirit of this kata so, in that case, return `None`/`nil`/`null`/`Nothing. Return {} with C++`. Return the string `"nil"` with Bash.

In that case the returned structure of **C** will have its `sz` component equal to `0`. (See the "Solution" and "Examples" tabs)

```rust
  sqInRect(5, 5) should return None
```

## Thinking

You can simply draw a rectangle. Then cut the rectangle with a square whose length is the length of the rectangle's shorter side. It will help you to understand.

**logic**

1. If `a` equals `b`, return `None`.
2. Compare `a` with `b`.
3. Push the smaller one into `Vec`.
4. Use the larger one minus the other one and we called the result `tmp`, which we consider it as `b`.
5. Repeat, until `a` equals zero.