## Detail

[Descending Order](https://www.codewars.com/kata/descending-order/train/rust)

Your task is to make a function that can take any non-negative integer as a argument and return it with its digits in descending order. Essentially, rearrange the digits to create the highest possible number.

\# Examples:

Input: `21445` Output: `54421`

Input: `145263` Output: `654321`

Input: `1254859723` Output: `9875543221`

## Thinking

1. Use `to_string()` covert `u64` to `String`.
2. Map the `String` then use `to_digit(10)` to covert `char` to `u32` (tpis: use `filter_map()` can ignore the unwrap()).
3. `sort()` it.
4. Covert it back to `u64`.