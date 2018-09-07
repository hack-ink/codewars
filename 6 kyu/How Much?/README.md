## Detail

[How Much?](https://www.codewars.com/kata/how-much/train/rust)

I always thought that my old friend John was rather richer than he looked, but I never knew exactly how much money he actually had. One day (as I was plying him with questions) he said: "Imagine I have between m and n Zloty (or did he say Quetzal? I can't remember!)

If I were to buy 9 cars costing `c` each, I'd only have 1 Zlotty (or was it Meticals?) left.

And if I were to buy 7 boats at `b` each, I'd only have 2 Ringglets (or was it Zlotty?) left.

Could you tell me in each possible case:

1. how much money `f` he could possibly have 
2. the cost `c` of a car 
3. the cost `b` of a boat?

So, I will have a better idea about his fortune. Note that if `m-n` is big enough, you might have a lot of possible answers. 

Each answer will be given as ["M: f", "B: b", "C: c"] and all the answers as [ ["M: f", "B: b", "C: c"] ... ]. M stands for "Money", B for boats, C for cars.

`m` and `n` are positive or null integers with m <= n or m >= n, m and n inclusive.

\##Examples:

```rust
howmuch(1, 100) => [["M: 37", "B: 5", "C: 4"], ["M: 100", "B: 14", "C: 11"]]
howmuch(1000, 1100) => [["M: 1045", "B: 149", "C: 116"]]
howmuch(10000, 9950) => [["M: 9991", "B: 1427", "C: 1110"]]
howmuch(0, 200) => [["M: 37", "B: 5", "C: 4"], ["M: 100", "B: 14", "C: 11"], ["M: 163", "B: 23", "C: 18"]]
```

Explanation of howmuch(1, 100):

In the first answer his possible fortune is 37 so if he buys 7 boats each worth 5 it remains `37 - 7 * 5 = 2`, if he buys 9 cars worth 4 each it remains `37 - 9 * 4 = 1`. The same with `f = 100: 100 - 7 * 14 = 2 and 100 - 9 * 11 = 1`.

\# Note

See "Sample Tests" to know the format of the return.

## Thinking

Actually, we have two linear equations:

```rust
Suppose:
    he has m Zlotty (or was it Meticals?).
    c Zlotty (or was it Meticals?) each car. 
    b Zlotty (or was it Meticals?) each boat.

Since:
    m - 9c = 1
    m - 7b = 2

Therefore:
    m % 9 = 1
    m % 7 = 2
```

