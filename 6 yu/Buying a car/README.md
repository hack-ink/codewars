## Detail

[Buying a car](https://www.codewars.com/kata/buying-a-car/train/rust)

A man has a rather old car being worth $2000. He saw a secondhand car being worth $8000. He wants to keep his old car until he can buy the secondhand one.

He thinks he can save $1000 each month but the prices of his old car and of the new one decrease of 1.5 percent per month. Furthermore the percent of loss increases by a **fixed 0.5** percent at the end of every two months.

**Can you help him?** Our man finds it difficult to make all these calculations.

How many months will it take him to save up enough money to buy the car he wants, and how much money will he have left over?

**Parameters and return of function:**

```rust
parameter (positive int, guaranteed) startPriceOld (Old car price)
parameter (positive int, guaranteed) startPriceNew (New car price)
parameter (positive int, guaranteed) savingperMonth 
parameter (positive float or int, guaranteed) percentLossByMonth

nbMonths(2000, 8000, 1000, 1.5) should return [6, 766] or (6, 766)
```

where 6 is the number of months at **the end of which** he can buy the new car and 766 is the nearest integer to '766.158...' .

**Note:** Selling, buying and saving are normally done at end of month. Calculations are processed at the end of each considered month but if, by chance from the start, the value of the old car is bigger than the value of the new one or equal there is no saving to be made, no need to wait so he can at the beginning of the month buy the new car:

```rust
nbMonths(12000, 8000, 1000, 1.5) should return [0, 4000]
nbMonths(8000, 8000, 1000, 1.5) should return [0, 0]
```

We don't take care of a deposit of savings in a bank:-)

## Thinking

|       old       |      new      | saving | prec | month | fixed |      sum      |
| :-------------: | :-----------: | :----: | :--: | :---: | :---: | :-----------: |
|      2,000      |     8,000     |   0    | 1.5  |   0   |   0   |    -6,000     |
|      1,970      |     7,880     | 1,000  | 1.5  |   1   |  0.5  |    -4,910     |
|     1,930.6     |    7,722.4    | 2,000  |  2   |   2   |  0.5  |   -3,791.8    |
|    1,891.988    |   7,567.952   | 3,000  |  2   |   3   |   1   |  -2,675.964   |
|   1,844.6883    |  7,378.7532   | 4,000  | 2.5  |   4   |   1   |  -1,534.0649  |
|  1,798.5710925  |  7,194.28437  | 5,000  | 2.5  |   5   |  1.5  | -395.7132775  |
| 1,744.613959725 | 6,978.4558389 | 6,000  |  3   |   6   |  1.5  | 766.158120825 |

**logic**

1. Adding 1000 to saving: `saving + 1000`.
2. Calculating new price: `old * prec`, `new * prec`.
3. Calculating sum: `old - new + saving`.
4. Judgment: `if month % 2 == 0 { prec + 0.5 }`. (Tips: 0 % 0 == 0)
5. Adding 1 to month: `month + 1`.
6. Judgment: `if sum >= 0` (Tips: if return sum, remember `round()` it)

