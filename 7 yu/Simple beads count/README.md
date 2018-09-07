## Detail

[Simple beads count](https://www.codewars.com/kata/simple-beads-count/train/rust)



Two red beads are placed between every two blue beads. There are N blue beads. After looking at the arrangement below work out the number of red beads.

<font color="red">@</font><font color="blue">@@</font><font color="red">@</font><font color="blue">@@</font><font color="red">@</font><font color="blue">@@</font><font color="red">@</font><font color="blue">@@</font><font color="red">@</font><font color="blue">@@</font><font color="red">@</font>

Implement `count_red_beads(n)` (in PHP `count_red_beads($n)`; in Java, Javascript, C, C++ `countRedBeads(n)`) so that it returns the number of red beads.
If there are less than 2 blue beads return 0.

## Thinking

Just care `attempt to subtract with overflow`.