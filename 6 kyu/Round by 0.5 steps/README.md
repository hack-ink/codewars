## Detail

[Round by 0.5 steps](https://www.codewars.com/kata/round-by-0-dot-5-steps/train/rust)

Round any given number to the closest 0.5 step

I.E.

```rust
solution(4.2) = 4
solution(4.3) = 4.5
solution(4.6) = 4.5
solution(4.8) = 5
```

Round **up** if number is as close to previous and next 0.5 steps.

```rust
solution(4.75) == 5
```

## Thinking

**Option 1:** match `< 0.25`, `< 0.75` and `_`.

**Option 2:** `(2.0 * n).round() / 2.0`.