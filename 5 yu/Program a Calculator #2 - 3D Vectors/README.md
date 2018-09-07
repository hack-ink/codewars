## Detail

[Program a Calculator #2 - 3D Vectors](https://www.codewars.com/kata/program-a-calculator-number-2-3d-vectors/train/rust)

\# Program a Calculator #2 - 3D Vectors

\# About

This Kata Series is aimed at representing mathematical concepts using code and writing programs to solve simple, typical mathematical problems. The solutions to these Kata may or may not be helpful in programming your own maths-based application.

\# Task

Declare and define a `Vector` class/struct which represents a vector in three-dimensional space. This class/struct should have three public (i.e. accessible anywhere within the program/script) properties `i`/`I`, `j`/`J` and `k`/`K`.

The `Vector` class/struct should have a class/struct constructor which accepts exactly three arguments, all of which are required, in the following order: `i`/`I`, `j`/`J`, `k`/`K`, and assign the values of the arguments to the public properties `i`/`I`, `j`/`J` and `k`/`K` respectively. It is expected that all three arguments will be valid numbers (integers and/or floats, doesn't matter) but no type checking is required on your part.

The magnitude of a `Vector` is often very important to know. For example, if the velocity of a car is represented by a three-dimensional `Vector`, its magnitude tells us the **speed** at which it is moving. Declare and define a public method `getMagnitude`/`GetMagnitude`/`get_magnitude`which accepts no arguments and returns the magnitude of the vector. Rounding will not be required as the test cases will allow for potential floating point errors.

The [unit vectors](https://en.wikipedia.org/wiki/Unit_vector) in the x, y and z directions of the coordinate axes are commonly denoted as **i**, **j** and **k** respectively. These three unit vectors are considered especially important in mathematics as they are the basis of the definitions of every other vector in 3D space. Define three public, static (i.e. directly invoked from the class itself) methods `getI`/`GetI`/`get_i`, `getJ`/`GetJ`/`get_j` and `getK`/`GetK`/`get_k`, each of which accepts no arguments and returns `Vector` objects representing the unit vectors **i**, **j**and **k** respectively.

A common vector operation is vector addition where two vectors are added together to give a resultant vector. Declare and define a public method `add`/`Add` which accepts exactly 1 required argument, another instance of `Vector`(no type checking required), and returns an instance of `Vector` which is the sum of the two vectors.

Another common vector operation is scalar multiplication where a vector is enlarged by a scalar. Declare and define a public method `multiplyByScalar`/`MultiplyByScalar`/`multiply_by_scalar`which accepts exactly 1 required argument, a number (integer or float, doesn't matter), and returns an instance of `Vector` which is the original vector multiplied by that scalar.

The [dot product](https://en.wikipedia.org/wiki/Dot_product), also commonly known as the **scalar product** of two vectors, is also very useful at determining things such as the angle between the two vectors. Declare and define a public method `dot`/`Dot` which accepts exactly 1 required argument, another instance of `Vector`(no type checking required), and returns the dot product between the two vectors. No rounding is necessary as the test cases will allow for potential floating point errors.

The [cross product](https://en.wikipedia.org/wiki/Cross_product) is often even more useful than the dot product as it gives a vector that is orthogonal (i.e. perpendicular) to both vectors involved. Another application of the cross product includes finding the area of a pallelogram formed by the two vectors. Declare and define a public method `cross`/`Cross` which accepts exactly 1 required argument, another instance of `Vector` (no type checking required), and returns an instance of `Vector` which is the cross product of the two vectors involved **in the order that the code would be read**. For example, `$a->cross($b)`/`a.cross(b)`/`a.Cross(b)` would return the result of `a × b` **NOT** `b × a`.

Now that we have defined our basic operations for our `Vector` class, it is time to move on to something more exciting. Declare and define a public method `isParallelTo`/`IsParallelTo`/`is_parallel_to`which receives exactly 1 required argument, another instance of `Vector` (no type checking required), and returns a boolean on whether the two vectors involved are parallel (or [antiparallel](https://en.wikipedia.org/wiki/Antiparallel_(mathematics)#Antiparallel_vectors)) to each other.  **Note, however, in the edge case where either (or both) vector is a zero vector, your method should return** `false`because you can't really define the direction of a zero vector, can you? Also note that your method should **account for potential floating point errors** if necessary. Don't worry, very small values (e.g. `<= 0.001`) will not be deliberately tested in the test cases to trip up your method ;)

It is also often very useful to know whether two vectors are perpendicular to each other. Declare and define a public method `isPerpendicularTo`/`IsPerpendicularTo`/`is_perpendicular_to`which accepts exactly 1 required argument, another instance of `Vector` (no type checking required), and returns a boolean on whether the two vectors involved are perpendicular to each other.  **Note, however, in the edge case where either (or both) vector is a zero vector, your method should return** `false`as you can't really define the direction of a zero vector, can you?  **This method should also be able to handle potential floating point errors correctly.**

Apart from the standard unit vectors **i**, **j** and **k**, other unit vectors can also prove to be very useful. Declare and define a public method `normalize`/`Normalize` which accepts no arguments and returns a [normalized version](https://en.wikipedia.org/wiki/Unit_vector) of said vector. The tests will allow for potential floating point errors.

Finally, declare and define a public method `isNormalized`/`IsNormalized`/`is_normalized`which accepts no arguments and returns a boolean stating whether the given vector is normalized (i.e. has unit length).  **Your method should account for potential floating point errors.**

\# Kata in this Series

1. [Program a Calculator #1 - Points in 3D Space](https://www.codewars.com/kata/program-a-calculator-number-1-points-in-3d-space)
2. **Program a Calculator #2 - 3D Vectors**

## Thinking

Easy kata but need some math knowledge.

By the way, you need to be careful in `is_parallel_to()` which you should handle many situations about `0`.