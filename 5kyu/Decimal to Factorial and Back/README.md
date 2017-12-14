## Detail

[Decimal to Factorial and Back](https://www.codewars.com/kata/decimal-to-factorial-and-back/train/rust)

If we are limited to digits 0...9 the biggest number we can code is 10! - 1.

So we extend 0..9 with letters A to Z. With these 36 digits we can code up to 36! − 1 = 37199332678990121746799944815083519999999910 (base 10)

We code two functions, the first one will code a decimal number and return a string with the factorial representation : "dec2FactString(nb)"

the second one will decode a string with a factorial representation and produce the decimal representation : "factString2Dec(str)".

Given numbers will be positive.

\# Note

You have tests with Big Integers in Clojure, Python, Ruby, Haskell, Ocaml but not with Java and others where the number "nb" in "dec2FactString(nb)" is at most a long.

**Ref:**<http://en.wikipedia.org/wiki/Factorial_number_system>

## Thinking

About `dec2_fact_string`:

1. Initialize a `HashMap` or a collection of tuple which like `Vec<(_, _)>` (you also can create a `Vec<char>` and use the index to pair the value). This can help you to find the value of a letter.
2. Initialize a collection of factorials. (or you can write a `fac()` function, but it will lost some efficiency)
3. Use `nb` to divide `fac` until the fac_collection is empty, if the result great than 10 that you should find its value in the alphabet_collection or you should covert it to `char`, then push it into the `String` which this function will return. In every loop, `nb = nb - nb / fac` (more simple: `nb %= fac`).

About `fact_string_2dec`:

1. It's same with the `dec2_fact_string`.
2. It's same with the `dec2_fact_string`. But you should swap the `k` and `v`.
3. Just use a `fold()` to calculate the sum of them.