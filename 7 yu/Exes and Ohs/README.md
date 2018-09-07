## Detail

[Exes and Ohs](https://www.codewars.com/kata/55908aad6620c066bc00002a)

Check to see if a string has the same amount of 'x's and 'o's. The method must return a boolean and be case insensitive. The string can contains any char.

Examples input/output:

```rust
XO("ooxx") => true
XO("xooxx") => false
XO("ooxXm") => true
XO("zpzpzpp") => true // when no 'x' and 'o' is present should return true
XO("zzoo") => false
```
## Thinking

Just count the `'x'`, `'X'`, `o` and `'O'`.