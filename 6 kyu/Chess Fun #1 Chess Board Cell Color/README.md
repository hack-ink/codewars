## Detail

[Chess Fun #1: Chess Board Cell Color](https://www.codewars.com/kata/chess-fun-number-1-chess-board-cell-color/train/rust)

\# Task

Given two cells on the standard chess board, determine whether they have the same color or not.

\# Example

For `cell1 = "A1" and cell2 = "C3"`, the output should be `true`.

![img](https://codefightsuserpics.s3.amazonaws.com/tasks/chessBoardCellColor/img/example1.png?_tm=1475149021926)

For `cell1 = "A1" and cell2 = "H3"`, the output should be `false`.

![img](https://codefightsuserpics.s3.amazonaws.com/tasks/chessBoardCellColor/img/example2.png?_tm=1475149022115)

\# Input/Output

- `[input]` string `cell1`

- `[input]` string `cell2`

- `[output]` a boolean value

  `true` if both cells have the same color, `false` otherwise.

## Thinking

See my code. There are 3 different ways. But the last 2 are similar.

1. Match each situation.
2. 1. Calculate sum than compare.
   2. Calculate remainder than compare.