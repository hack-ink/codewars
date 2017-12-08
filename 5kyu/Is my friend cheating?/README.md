## Detail

[Is my friend cheating?](https://www.codewars.com/kata/is-my-friend-cheating/train/rust)

- A friend of mine takes a sequence of numbers from 1 to n (where n > 0).
- Within that sequence, he chooses two numbers, a and b.
- He says that the product of a and b should be equal to the sum of all numbers in the sequence, excluding a and b.
- Given a number n, could you tell me the numbers he excluded from the sequence?

The function takes the parameter: `n` (don't worry, n is always strictly greater than 0 and small enough so we shouldn't have overflow) and returns an array of the form: 

```rust
[(a, b), ...] or [[a, b], ...] or {{a, b}, ...} or or [{a, b}, ...]
```

with **all** `(a, b)` which are the possible removed numbers in the sequence `1 to n`.

`[(a, b), ...] or [[a, b], ...] or {{a, b}, ...} or ...`will be sorted in increasing order of the "a".

It happens that there are several possible (a, b). The function returns an empty array if no possible numbers are found which will prove that my friend has not told the truth! (Go: in this case return `nil`).

(See examples for each language in "RUN EXAMPLES")

\#Examples:

```rust
removNb(26) should return [(15, 21), (21, 15)]
```

or

```rust
removNb(26) should return { {15, 21}, {21, 15} }
```

or

```rust
removeNb(26) should return [[15, 21], [21, 15]]
```

or

```rust
removNb(26) should return [ {15, 21}, {21, 15} ]
```

or

```rust
in C:
removNb(26) should return  **an array of pairs {{15, 21}{21, 15}}**
tested by way of strings.
```

## Thinking

Find `a` and `b`, which meet the requirement below:

```rust
Since:
    sum of all numbers in the sequence - a - b = a * b

Therefore:
    b = (sum of all numbers in the sequence - a) / (a + 1) (tips: b need to be a integer)
    use a to make a loop. (tips: a always smaller then the smallest b which in the result, so you can avoid some unnecessary loop)
    push() (a, b) and (b, a) into result in one time that will be more efficient. Or you can append() vec![(a, b), (b, a)].
```

