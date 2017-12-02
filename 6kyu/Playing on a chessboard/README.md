## Detail

[Playing on a chessboard](https://www.codewars.com/kata/playing-on-a-chessboard/train/rust)

`1/4, 2/5, 3/6, 4/7, 5/8, 6/9, 7/10, 8/11`

until last row:

`1/9, 2/10, 3/11, 4/12, 5/13, 6/14, 7/15, 8/16`

When all numbers are on the chessboard each in turn we toss a coin. The one who get "head" wins and the other gives him, in dollars, the **sum of the numbers on the chessboard**. We play for fun, the dollars come from a monopoly game! 

How much can I (or my friend) win or loses for each game if the chessboard has n rows and n columns? Add all of the fractional values on an n by n sized board and give the answer as a simplified fraction.

Ruby, Python, JS, Coffee, Clojure, PHP, Elixir, Crystal, Typescript, Go:

The function called 'game' with parameter n (integer >= 0) returns as result an irreducible fraction written as an array of integers: [numerator, denominator]. If the denominator is 1 return [numerator].

- Haskell:

'game' returns either a "Left Integer" if denominator is 1 otherwise "Right (Integer, Integer)" 

- Java, C#, C++, F#, Swift:

'game' returns a string that mimicks the array returned in Ruby, Python, JS, etc...

- Bash returns a string

(see Example Test Cases for each language)

## Thinking

Use a pen and a paper, write down some example you will find it rhythmical.

`vec![n * n, 2]`.