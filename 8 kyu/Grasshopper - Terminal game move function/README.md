## Detail

[Grasshopper - Terminal game move function](https://www.codewars.com/kata/563a631f7cbbc236cf0000c2)

In this game, the hero moves from left to right. The player rolls the dice and moves the number of spaces indicated by the dice **two times**.

Create a function for the terminal game that takes the current position of the hero and the roll (1-6) and return the new position.

\# Example:

```rust
move_hero(3, 6) -> 15
```

## Thinking

Position: `position + roll + roll`.