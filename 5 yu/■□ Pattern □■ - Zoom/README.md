## Detail

[■□ Pattern □■ : Zoom](https://www.codewars.com/kata/number-number-pattern-number-number-zoom/train/rust)

\# Task:

- #### Complete the pattern, using the special characters `■ □`

- #### In this kata, let's us draw a square, with a zoom effect.

\# Rules:

- parameter `n`: The side length of the square, always be a positive odd number.
- return a string square that `■` and `□` alternate expansion and each line is separated by "\n".

\# Example:

```rust
zoom(1)

  ■

zoom(3)

□□□
□■□
□□□

zoom(5)

■■■■■
■□□□■
■□■□■
■□□□■
■■■■■

zoom(7)

□□□□□□□
□■■■■■□
□■□□□■□
□■□■□■□
□■□□□■□
□■■■■■□
□□□□□□□

zoom(9)

■■■■■■■■■
■□□□□□□□■
■□■■■■■□■
■□■□□□■□■
■□■□■□■□■
■□■□□□■□■
■□■■■■■□■
■□□□□□□□■
■■■■■■■■■

zoom(25)

■■■■■■■■■■■■■■■■■■■■■■■■■
■□□□□□□□□□□□□□□□□□□□□□□□■
■□■■■■■■■■■■■■■■■■■■■■■□■
■□■□□□□□□□□□□□□□□□□□□□■□■
■□■□■■■■■■■■■■■■■■■■■□■□■
■□■□■□□□□□□□□□□□□□□□■□■□■
■□■□■□■■■■■■■■■■■■■□■□■□■
■□■□■□■□□□□□□□□□□□■□■□■□■
■□■□■□■□■■■■■■■■■□■□■□■□■
■□■□■□■□■□□□□□□□■□■□■□■□■
■□■□■□■□■□■■■■■□■□■□■□■□■
■□■□■□■□■□■□□□■□■□■□■□■□■
■□■□■□■□■□■□■□■□■□■□■□■□■
■□■□■□■□■□■□□□■□■□■□■□■□■
■□■□■□■□■□■■■■■□■□■□■□■□■
■□■□■□■□■□□□□□□□■□■□■□■□■
■□■□■□■□■■■■■■■■■□■□■□■□■
■□■□■□■□□□□□□□□□□□■□■□■□■
■□■□■□■■■■■■■■■■■■■□■□■□■
■□■□■□□□□□□□□□□□□□□□■□■□■
■□■□■■■■■■■■■■■■■■■■■□■□■
■□■□□□□□□□□□□□□□□□□□□□■□■
■□■■■■■■■■■■■■■■■■■■■■■□■
■□□□□□□□□□□□□□□□□□□□□□□□■
■■■■■■■■■■■■■■■■■■■■■■■■■
```

## Thinking

I think the most efficient is calculate it in half. For example:

```rust
1 1 1 1 1
1 0 0 0 1
1 0 1 0 1
1 0 0 0 1
1 1 1 1 1

=>

top = [
    1 1 1 1 1
    1 + 000 + reverse 1
]

mid = 1 0 + 1 + reverse 1 0

bottom = reverse half 
```

