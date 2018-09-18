## Detail
[Beginner - Reduce but Grow](https://www.codewars.com/kata/beginner-reduce-but-grow/train/rust)
Given an array of integers (x), return the result of multiplying the values together in order. Example:

```
[1, 2, 3] --> 6
```

For the beginner, try to use the reduce method - it comes in very handy quite a lot so it's a good one to know.

Array will not be empty.

## Thinking

`array.into_iter().fold(1, |acc, x| acc * x)`.