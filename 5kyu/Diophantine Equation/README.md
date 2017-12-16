## Detail

[Diophantine Equation](https://www.codewars.com/kata/diophantine-equation/train/rust)

In mathematics, a Diophantine equation is a polynomial equation, usually in two or more unknowns, such that only the integer solutions are sought or studied.

In this kata we want to find all integers x, y (x >= 0, y >= 0) solutions of a diophantine equation of the form 

```rust
 x ^ 2 - 4 * y ^ 2 = n
```

where the unknowns are x and y and n is a given positive number. Solutions x, y will be given as an array of arrays (Ruby, Python, Clojure, JS, CS, TS)

```rust
 [[x1, y1], [x2, y2] ....]
```

as an array of tuples (Haskell, C++, Elixir)

```rust
 [(x1, y1), (x2, y2) ....] or { {x1, y1}, {x2, y2} ....} or [{x1, y1}, {x2, y2} ....]
```

as an array of pairs (C, see example tests)

```rust
{{x1, y1}{x2, y2} ....}
```

and as a string (Java, C#)

```rust
 "[[x1, y1], [x2, y2] ....]"
```

in decreasing order of the positive xi. If there is no solution returns `[]` or `"[]"`.

**Examples:**

```rust
sol_equa(90005) -->  [[45003, 22501], [9003, 4499], [981, 467], [309, 37]]

sol_equa(90002) --> []

(Java, C#)

solEquaStr(90005) --> "[[45003, 22501], [9003, 4499], [981, 467], [309, 37]]"

solEquaStr(90002) --> "[]"
```

**Hint:** `x ^ 2 - 4 * y ^ 2 = (x - 2 * y) * (x + 2 * y)`.

## Thinking

Just slove `(x - 2 * y) * (x + 2 * y)`. But you need some mathematic theory to optimize.