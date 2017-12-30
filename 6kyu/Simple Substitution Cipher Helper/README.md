## Detail

[Simple Substitution Cipher Helper](https://www.codewars.com/kata/52eb114b2d55f0e69800078d)

A simple substitution cipher replaces one character from an alphabet with a character from an alternate alphabet, where each character's position in an alphabet is mapped to the alternate alphabet for encoding or decoding.

E.g.

```rust
let map1 = "abcdefghijklmnopqrstuvwxyz";
let map2 = "etaoinshrdlucmfwypvbgkjqxz";

let cipher = Cipher::new(map1, map2);
cipher.encode("abc") // => "eta"
cipher.encode("xyz") // => "qxz"
cipher.encode("aeiou") // => "eirfg"

cipher.decode("eta") // => "abc"
cipher.decode("qxz") // => "xyz"
cipher.decode("eirfg") // => "aeiou"
```

If a character provided is not in the opposing alphabet, simply leave it as be.

## Thinking

Use `zip()` in `Cipher::new()` and combine `find()` with `map_or()` in encode(decode).

