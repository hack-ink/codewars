## Detail

[k-Primes](https://www.codewars.com/kata/5726f813c8dcebf5ed000a6b)

A natural number is called **k-prime** if it has exactly k prime factors, counted with multiplicity.

A natural number is thus prime if and only if it is 1-prime.

\# Examples:

```rust
k = 2 -> 4, 6, 9, 10, 14, 15, 21, 22, …
k = 3 -> 8, 12, 18, 20, 27, 28, 30, …
k = 5 -> 32, 48, 72, 80, 108, 112, …
```

\# Task:

- Write function `count_Kprimes (or countKprimes or count-K-primes or kPrimes in C)` which given parameters `k, start, end (or nd)` returns an **array** (or a list in C) of the `k-primes` between `start (inclusive)` and `end (inclusive)`.

\# Example:

```rust
countKprimes(5, 500, 600) --> [500, 520, 552, 567, 588, 592, 594]
```

...............................................................................

- **for all languages except Bash shell**

Given positive integers `s` and `a, b, c` where `a is 1-prime`, `b 3-prime`, `c 7-prime` find the **number** of solutions of `a + b + c = s`. Call this function `puzzle(s)`.

\# Examples:

```rust
puzzle(138) --> 1 ([2 + 8 + 128] is solution)
puzzle(143) --> 2 ([3 + 12 + 128, 7 + 8 + 128] are solutions)
```

...............................................................................

\# Notes: 

- The first function would have been better named: `findKprimes` or `kPrimes` :-)
- In C some helper functions are given (see declarations in 'Solution').
- For Go: nil slice is expected when there are no k-primes between `start` and `end`.

## Thinking

