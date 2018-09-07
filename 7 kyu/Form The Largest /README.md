## Detail
[Form The Largest ](https://www.codewars.com/kata/form-the-largest/train/rust)
\# Task 

**_Given_**   *a number* , **_Return_**  **_The Maximum number _**  *could be formed from the digits of the number given* . 
___
\# Notes

* **_Only Positve numbers_** *passed to the function , numbers Contain digits [1:9] inclusive*  ![!alt](https://i.imgur.com/mdX8dJP.png)
![!alt](https://i.imgur.com/mdX8dJP.png) 

* **_Digit Duplications_** *could occur* , So also **_consider it when forming the Largest_**   ![!alt](https://i.imgur.com/mdX8dJP.png) 

____
\# Input >> Output Examples:

```cpp
1- maxNumber (213) ==> return (321)
```
\## **_Explanation_**:

As `321` is **_The Maximum number _**  *could be formed from the digits of the number   **_213_*** . 
___

```cpp
2- maxNumber (7389) ==> return (9873)
```
\## **_Explanation_**:

As `9873` is **_The Maximum number _**  *could be formed from the digits of the number  **_7389_*** . 
___

```cpp
3- maxNumber (63729) ==> return (97632)
```
\## **_Explanation_**:

As `97632` is **_The Maximum number _**  *could be formed from the digits of the number  **_63729_*** . 
___

```cpp
4- maxNumber (566797) ==> return (977665)
```
\## **_Explanation_**:

As `977665` is **_The Maximum number _**  *could be formed from the digits of the number  **_566797_*** .

**_Note_** : **_Digit duplications are considered when forming the largest_** . 
___

```cpp
5- maxNumber (17693284) ==> return (98764321)
```
\## **_Explanation_**:

As `98764321` is **_The Maximum number _**  *could be formed from the digits of the number  **_17693284_*** .
___
___
___

\# [Playing with Numbers Series](https://www.codewars.com/collections/playing-with-numbers)

\# [Playing With Lists/Arrays Series](https://www.codewars.com/collections/playing-with-lists-slash-arrays)

\# [For More Enjoyable Katas](http://www.codewars.com/users/MrZizoScream/authored)
___

\## ALL translations are welcomed

\## Enjoy Learning !!
\# Zizou

## Thinking

`to_vec()` -> `sort()` -> `parse()`.