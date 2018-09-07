## Detail

[Steps in k-primes](https://www.codewars.com/kata/5a48948e145c46820b00002f)

A natural number is called **k-prime** if it has exactly k prime factors, counted with multiplicity.

A natural number is thus prime if and only if it is 1-prime.

```rust
Examples of k-primes:
k = 2 -> 4, 6, 9, 10, 14, 15, 21, 22, …
k = 3 -> 8, 12, 18, 20, 27, 28, 30, …
k = 5 -> 32, 48, 72, 80, 108, 112, …
```

The k-prime numbers are not regularly spaced. Between 2 and 50 we have the following 2-primes:

`[4, 6, 9, 10, 14, 15, 21, 22, 25, 26, 33, 34, 35, 38, 39, 46, 49]`

The `steps` between two consecutive k-primes are `2, 3, 1, 4, 1, 6, 1, 3, 1, 7, 1, 1, 3, 1, 7, 3`.

We will write a function `kprimes_step(k, step, start, nd)` with parameters:

-   `k` (integer > 0) which indicates the type of k-primes we are looking for,
-   `step` (integer > 0) which indicates the `step` we want to find betwwen two k-primes,
-   `start` (integer >= 0) which gives the start of the search (start inclusive),
-   `nd` (integer >= start) which gives the end of the search (nd inclusive)

In the example above `kprimes_step(2, 2, 0, 50)` will return `[[4, 6], [33, 35]]` which are the pairs of 2-primes between 2 and 50 with a 2 steps.

So this function should return an array of all the pairs of k-prime numbers spaced with a step of `step` between the limits `start`, `nd`. This array can be empty.

\#Examples:

```rust
kprimes_step(2, 2, 0, 50) => [[4, 6], [33, 35]]
kprimes_step(6, 14, 2113665, 2113889) => [[2113722, 2113736]])
kprimes_step(2, 10, 0, 50) => [[4, 14], [15, 25], [25, 35], [39, 49]]
kprimes_step(5, 20, 0, 50) => []
```

## Thinking

The algorithm isn't so hard, so I won't focus on it.

To solve this kata you should have a clearly logic:

1.  Find the **kprimes**.
    1.  Find the number which matches the requirements. (function: `prime_factors`)
    2.  Collect them. (function: `count_kprimes`)
2.  Check the `step`.
    1.  Calculate the difference `primes[y] - primes[x]` *(primes[y] > primes[x])*. (If you want it more efficiency, remove the number which less than or equal to the `primes[x]`)
3.  Return the result.