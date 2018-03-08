## Detail

[Digital cypher](https://www.codewars.com/kata/digital-cypher)

\# Introduction

Digital Cypher assigns to each letter of the alphabet unique number. For example:

```rust
 a  b  c  d  e  f  g  h  i  j  k  l  m
 1  2  3  4  5  6  7  8  9 10 11 12 13
 n  o  p  q  r  s  t  u  v  w  x  y  z
14 15 16 17 18 19 20 21 22 23 24 25 26
```

Instead of letters in encrypted word we write the corresponding number, eg. The word scout:

```rust
 s  c  o  u  t
19  3 15 21 20
```

Then we add to each obtained digit consecutive digits from the key. For example. In case of key equal to `1939` :

```rust
   s  c  o  u  t
  19  3 15 21 20
 + 1  9  3  9  1
 ---------------
  20 12 18 30 21

   m  a  s  t  e  r  p  i  e  c  e
  13  1 19 20  5 18 16  9  5  3  5
+  1  9  3  9  1  9  3  9  1  9  3
  --------------------------------
  14 10 22 29  6 27 19 18  6  12 8
```

\# Task

Write a function that accepts `str` string and `key` number and returns an array of integers representing encoded `str`.

\# Input / Output

The `str` input string consists of lowercase characters only.
The `key` input number is a positive integer.

\# Example

```rust
Encode("scout",1939);  ==>  [ 20, 12, 18, 30, 21]
Encode("masterpiece",1939);  ==>  [ 14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]
```

\# Digital cypher series

-   [Digital cypher vol 1](https://www.codewars.com/kata/592e830e043b99888600002d)
-   [Digital cypher vol 2](https://www.codewars.com/kata/592edfda5be407b9640000b2)
-   [Digital cypher vol 3 - missing key](https://www.codewars.com/kata/5930d8a4b8c2d9e11500002a)

## Thinking

Use `cycle()` to make a loop. There is a trick, convert `char` to `i32`~ 