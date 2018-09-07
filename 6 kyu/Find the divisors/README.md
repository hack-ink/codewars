## Detail

[Find the divisors!](https://www.codewars.com/kata/544aed4c4a30184e960010f4)

Create a function named `divisors`/`Divisors` that takes an integer and returns an array with all of the integer's divisors(except for 1 and the number itself). If the number is prime return the string '(integer) is prime' (`null` in C#) (use `Either String a` in Haskell and `Result<Vec<u32>, String>` in Rust).

\# Example:

```rust
divisors(12); // should return Ok(vec![2,3,4,6])
divisors(25); // should return Ok(vec![5])
divisors(13); // should return Err("13 is prime")
```

You can assume that you will only get positive integers as inputs.

## Thinking

Find the divs from a **range**.