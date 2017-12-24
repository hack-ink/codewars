## Detail

[John and Ann sign up for Codewars](https://www.codewars.com/kata/john-and-ann-sign-up-for-codewars/train/rust)

John and his wife Ann have decided to go to Codewars. 

On day 0 Ann will do one kata and John - he wants to know how it is working - 0.

Let us call `a(n)` the number of katas done by Ann at day `n` we have `a(0) = 1` and in the same manner `j(0) = 0`.

They have chosen the following rules:

- On day `n` the number of katas done by Ann should be `n` minus the number of katas done by John at day `t`, `t` being equal to the number of katas done by Ann herself at day `n - 1`.
- On day `n` the number of katas done by John should be `n` minus the number of katas done by Ann at day `t`, `t` being equal to the number of katas done by John himself at day `n - 1`.

Whoops! I think they need to lay out a little clearer exactly what there're getting themselves into!

\# Could you write:

- 1) two functions `ann` and `john (parameter n)` giving the list of the numbers of katas Ann and John should take on each day from day `0` to day `n - 1` (n days - see first example below)? 
- 2) The total number of katas taken by `ann` (function `sum_ann(n))` and `john`(function `sum_john(n))` from day `0` (inclusive) to day `n` (exclusive)? 

\# Examples:

```rust
john(11) -->  [0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6]
ann(6) -->  [1, 1, 2, 2, 3, 3]

sum_john(75) -->  1720
sum_ann(150) -->  6930
```

\# Shell Note:

sumJohnAndAnn has two parameters:

first one : n (number of days, $1)

second one : which($2) ->

- 1 for getting John's sum
- 2 for getting Ann's sum.

See "Sample Tests".

\# Note:

Keep an eye on performance.

## Thinking

a:

```rust
Since:
    t = a(n - 1)
    a(n) = n - j(t)

Therefore:
    a(n) = n - j(a(n - 1))
```

j:

```rust
Since:
    t = j(n - 1)
    j(n) = n - a(t)
    
Therefore:
    j(n) = n - a(j(n - 1))
```

|  n   | a's t | a(n) | j's t | j(n) |
| :--: | :---: | :--: | :---: | :--: |
|  0   |   -   |  1   |   -   |  0   |
|  1   |   1   |  1   |   0   |  0   |
|  2   |   1   |  2   |   0   |  1   |
|  3   |   2   |  2   |   1   |  2   |
|  4   |   2   |  3   |   2   |  2   |
|  5   |   3   |  3   |   2   |  3   |
|  6   |   3   |  4   |   3   |  4   |
|  7   |   4   |  5   |   4   |  4   |
|  8   |   5   |  5   |   4   |  5   |
|  9   |   5   |  6   |   5   |  6   |
|  10  |   6   |  6   |   6   |  6   |
|  11  |   6   |  7   |   6   |  7   |
|  12  |   7   |  8   |   7   |  7   |
|  13  |   8   |  8   |   7   |  8   |
|  14  |   8   |  9   |   8   |  9   |
|  15  |   9   |  9   |   9   |  9   |
|  16  |   9   |  10  |   9   |  10  |
|  17  |  10   |  11  |  10   |  11  |
|  18  |  11   |  11  |  11   |  11  |

Tips: Use 2 collections to store **an** and **jn**.