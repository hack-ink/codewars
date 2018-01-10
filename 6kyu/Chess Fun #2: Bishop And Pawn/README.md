## Detail

[Chess Fun #2: Bishop And Pawn](https://www.codewars.com/kata/589425c2561a35dd1a0000a2/solutions/rust)



\# Task

Given the positions of a white `bishop` and a black `pawn` on the standard chess board, determine whether the bishop can capture the pawn in one move.

The `bishop` has no restrictions in distance for each move, but is limited to diagonal movement. Check out the example below to see how it can move:

![img](https://codefightsuserpics.s3.amazonaws.com/tasks/bishopAndPawn/img/bishop.jpg?_tm=1473536699597)

\# Example

For `bishop = "a1" and pawn = "c3"`, the output should be `true`.

![img](https://codefightsuserpics.s3.amazonaws.com/tasks/bishopAndPawn/img/ex1.jpg?_tm=1473536699893)

For `bishop = "h1" and pawn = "h3"`, the output should be `false`.

![img](https://codefightsuserpics.s3.amazonaws.com/tasks/bishopAndPawn/img/ex2.jpg?_tm=1473536700199)

\# Input/Output

-   `[input]` string `bishop`

    Coordinates of the white bishop in the chess notation.


-   `[input]` string `pawn`

    Coordinates of the black pawn in the same notation.


-   `[output]` a boolean value

    `true` if the bishop can capture the pawn, `false` otherwise.

## Thinking

Divide the string into 2 parts. 

`"a0" -> 'a' and '0'`

`"c3" -> 'c' and '3'`

Think about `(3 - 0).abs() == (c - a).abs()` mean what?