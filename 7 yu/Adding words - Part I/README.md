## Detail

[Adding words - Part I](https://www.codewars.com/kata/adding-words-part-i/train/rust)

This is the first part of this kata series. Second part is [here](https://www.codewars.com/kata/adding-words-part-ii/) and third part is [here](https://www.codewars.com/kata/adding-words-part-iii/)

Add two English words together!

Implement a class `Arith` (struct `struct Arith{value : &'static str,}` in Rust) such that

```rust
  //javascript
  var k = new Arith("three");
  k.add("seven"); //this should return "ten"
```

```rust
  //c++
  Arith* k = new Arith("three");
  k->add("seven"); //this should return string "ten"
```

```rust
  //Rust
  let c = Arith{value: "three"};
  c.add("seven") // this should return &str "ten"
```

**Input** - Will be between zero and ten and will always be in lower case

**Output** - Word representation of the result of the addition. Should be in lower case

## Thinking

Use a `[&'static str]` to store "string number" and consider those string's index as value.