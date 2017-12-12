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

