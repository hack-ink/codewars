## Detail

[Bouncing Balls](https://www.codewars.com/kata/bouncing-balls/train/rust)

A child plays with a ball on the nth floor of a big building. The height of this floor is known. 

`(float parameter "h" in meters. Condition 1) : h > 0)`

He lets out the ball. The ball bounces for example to two-thirds of its height.

`(float parameter "bounce". Condition 2) : 0 < bounce < 1)`

His mother looks out of a window that is 1.5 meters from the ground.

`(float parameters "window". Condition 3) : window < h).`

How many times will the mother see the ball **either** falling **or** bouncing in front of the window?

`If all three conditions above are fulfilled, return a positive integer, otherwise return -1.`

**Note**

You will admit that the ball can only be seen if the height of the rebouncing ball is stricty greater than the window parameter.

**Example:**

> h = 3, bounce = 0.66, window = 1.5, result is 3
>
> h = 3, bounce = 1, window = 1.5, result is -1 (Condition 2) not fulfilled).

## Thinking

Simple kata, but there's two thing you should know:

1. At the beginning, times = 1.
2. Whenever the ball bouncing or falling, times += 1. 