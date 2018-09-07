## Detail

[Make the Deadfish swim.](https://www.codewars.com/kata/make-the-deadfish-swim/train/rust)

Write a simple parser that will parse and run Deadfish.
Deadfish has 4 commands, each 1 character long.
'i' will increment the value ( initially 0 ).
'd' will decrement the value.
's' will square the value.
'o' will output the value into the return array.
Invalid characters should be ignored. 

```rust
parse( "iiisdoso" ); // should == [ 8 , 64 ]
```

## Thinking

Just match `'i'`, `'d'`, `'s'`, `'o'` and remember handle the exceptional case.