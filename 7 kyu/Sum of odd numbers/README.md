## Detail

[Sum of odd numbers](https://www.codewars.com/kata/55fd2d567d94ac3bc9000064)

Given the triangle of consecutive odd numbers:

```rust
             1
          3     5
       7     9    11
   13    15    17    19
21    23    25    27    29
...
```

Calculate the row sums of this triangle from the row index (starting at index 1) e.g.:

```rust
row_sum_odd_numbers(1); # 1
row_sum_odd_numbers(2); # 3 + 5 = 8
```

## Thinking

Always equals to n cubic.