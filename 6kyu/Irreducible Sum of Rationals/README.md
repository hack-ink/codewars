## Detail

[Irreducible Sum of Rationals](https://www.codewars.com/kata/irreducible-sum-of-rationals/train/rust)

You will have a list of rationals in the form

m = [ [numer_1, denom_1] , ... , [numer_n, denom_n] ] or m = [ (numer_1, denom_1) , ... , (numer_n, denom_n) ]

where all numbers are positive integers. You have to produce the sum N/D of these rationals in an irreducible form ie N and D have only 1 for divisor. 

The result will be written in the form 

- `[N, D]` in Ruby/Python/Clojure/JS/CS/PHP
- `Just "N D"` in Haskell or `Some "N D"` in F# 
- `Some((N, D)) in Rust`
- `"[N, D]"` in Java, CSharp, TS
- `{N, D}` in C++
- `{N, D}` in Elixir
- c(N, D) in R

\#Example:

```rust
[ [1, 2], [1, 3], [1, 4] ] ----> 
[13, 12] or: Just "13 12" (Haskell) or: "[13, 12]" (Java, CSharp, TS) or: {13, 12} (C++, Elixir), (Some "13 12") Fsharp, (13, 12) Swift,
c(13, 12) R
```

\#Notes:

- If m is `[]` return 

  - `nil/None/null`(Ruby/Elixir/Clojure/Python/JS/CS/TS/Java/CSharp/PHP/Swift/R) 
  - `Nothing` (Haskell) `None` (F#, Rust)
  - `{0, 1}` (C++, C)

- If D divise N return:

  ```rust
  N/D as an integer: n (Ruby/Elixir/Clojure/Python/JS/CS/PHP/R)
  Just "n" (Haskell), Some "n" (Haskell), "n" (Java, CSharp, TS), {n, 1} (C++, C)
  Some((n, 1)) (Rust), (Some "n") Fsharp, (n, 1) Swift
  ```

- See: Example Tests for more examples

## Thinking

