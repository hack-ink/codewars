## Detail

[The 'if' function](https://www.codewars.com/kata/54147087d5c2ebe4f1000805)

Who likes keywords? Nobody likes keywords, so why use them?

You know what keyword I use too much? `if`! We should **make a function** called `_if`, with its **arguments as a logical test and two functions/lambdas where the first function is executed if the boolean is true, and the second if it's false**, like an if/else statement, so that we don't have to mess around with those nasty keywords! Even so, **It should support truthy/falsy types** just like the keyword.

\# Example:

```rust
_if(true, || println!("true"), || println!("false"))
# prints "true" to the console
```
## Thinking

`judgement { return then()/els(); }`