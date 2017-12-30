## Detail

[Twice linear](https://www.codewars.com/kata/twice-linear/train/rust)

Consider a sequence `u` where u is defined as follows:

1. The number `u(0) = 1` is the first one in `u`.
2. For each `x` in `u`, then `y = 2 * x + 1` and `z = 3 * x + 1` must be in `u` too.
3. There are no other numbers in `u`.

Ex: `u = [1, 3, 4, 7, 9, 10, 13, 15, 19, 21, 22, 27, ...]`

1 gives 3 and 4, then 3 gives 7 and 10, 4 gives 9 and 13, then 7 gives 15 and 22 and so on...

\#Task: Given parameter `n` the function `dbl_linear` (or dblLinear...) returns the element `u(n)` of the ordered (with <) sequence `u`.

\#Example: `dbl_linear(10) should return 22`

\#Note: Focus attention on efficiency

## Thinking

Two choice (for my solution):

1. Use `Deque`, `push_back()`, `pop_front()`.
2. Use `Vec`, `push()`, `remove(0)`.

|  x   |                    y                     |                    z                     |
| :--: | :--------------------------------------: | :--------------------------------------: |
|  1   |                    []                    |                    []                    |
|  1   |                   [1]                    |                   [1]                    |
|  1   |                [~~1~~, 3]                |                [~~1~~, 4]                |
|  3   |            [~~1~~, ~~3~~, 7]             |              [~~1~~, 4, 10]              |
|  4   |           [~~1~~, ~~3~~, 7, 9]           |          [~~1~~, ~~4~~, 10, 13]          |
|  7   |       [~~1~~, ~~3~~, ~~7~~, 9, 15]       |        [~~1~~, ~~4~~, 10, 13, 22]        |
|  9   |   [~~1~~, ~~3~~, ~~7~~, ~~9~~, 15, 19]   |      [~~1~~, ~~4~~, 10, 13, 22, 28]      |
|  10  | [~~1~~, ~~3~~, ~~7~~, ~~9~~, 15, 19, 21] |  [~~1~~, ~~4~~, ~~10~~, 13, 22, 28, 31]  |
|  13  | [~~1~~, ~~3~~, ~~7~~, ~~9~~, 15, 19, 21, 27] | [~~1~~, ~~4~~, ~~10~~, ~~13~~, 22, 28, 31, 40] |

