## Detail

[Simple Interactive Interpreter](https://www.codewars.com/kata/52ffcfa4aff455b3c2000750)

\# Task:

You will create an interpreter which takes inputs described below and produces outputs, storing state in between each input.

If you're not sure where to start with this kata, check out my [Simpler Interactive Interpreter](http://www.codewars.com/dojo/katas/53005a7b26d12be55c000243) kata, which greatly simplifies the interpreter by removing functions.

**Note that the** `eval` **command has been disabled.**

\# Concepts:

The interpreter will take inputs in the language described under the language header below. This section will give an overview of the language constructs.

\# Variables:

Any `identifier` which is not a keyword or a function name will be treated as a variable. If the identifier is on the left hand side of an assignment operator, the result of the right hand side will be stored in the variable. If a variable occurs as part of an expression, the value held in the variable will be substituted when the expression is evaluated.

Variables are implicitly declared the first time they are assigned to.

**Example:** Initializing a variable to a constant value and using the variable in another expression (Each line starting with a '>' indicates a separate call to the input method of the interpreter, other lines represent output)

```rust
>x = 7
    7
>x + 6
    13
```

Referencing a non-existent variable will cause the interpreter to throw an error. The interpreter should be able to continue accepting input even after throwing.

**Example:** Referencing a non-existent variable

```rust
>y + 7
    ERROR: Invalid identifier. No variable with name 'y' was found."
```

\# Assignments:

An assignment is an expression that has an identifier on left side of an `=` operator, and any expression on the right. Such expressions should store the value of the right hand side in the specified variable and return the result.

**Example:** Assigning a constant to a variable

```rust
x = 7
    7
```

You should also be able to chain and nest assignments. Note that the assignment operator is one of the few that is right associative.

**Example:** Chained assignments. The statement below should set both `x` and `y` to `7`.

```rust
x = y = 7
    7
```

**Example:** Nested assignments. The statement below should set `y` to `3`, but it only outputs the final result.

```rust
x = 13 + (y = 3)
    16
```

\# Operator Precedence:

Operator precedence will follow the common order. There is a table in the *Language* section below that explicitly states the operators and their relative precedence.

\# Functions:

Functions are declared by the `fn` keyword followed by a name, an optional arguments list, the `=>` operator, and finally an expression. All function variables are local to the function. That is, the only variable names allowed in the function body are those declared by the arguments list. If a function has an argument called 'x', and there is also a global variable called 'x', the function should use the value of the supplied argument, not the value of the global variable, when evaluating the epxression. References to variables not found in the argument list should result in an error when the function is defined.

**Example:** declare a function to calculate the average of two variables and call it. (Each line starting with a '>' indicates a separate call to the input method of the interpreter, other lines represent output)

```rust
>fn avg => (x + y) / 2
    ERROR: Unknown identifier 'x'
>fn avg x y => (x + y) / 2
>a = 2
    2
>b = 4
    4
>avg a b
    3
```

**Example:** declare a function with an invalid variable name in the function body

```rust
>fn add x y => x + z
    ERROR: Invalid identifier 'z' in function body.
```

**Example:** chain method calls (hint: function calls are right associative!)

```rust
>fn echo x => x
>fn add x y => x + y
>add echo 4 echo 3
    7
```

\# Name conflicts:

Because variable and function names share the same grammar, conflicts are possible. Precedence will be given to the first object declared. That is, if a variable is declared, then subsequent declaration of a function with the same name should result in an error. Likewise, declaration of a function followed by the initialization of a variable with the same name should result in an error.

Declaration of function with the same name as an existing function should overwrite the old function with the new one.

**Example:** Overwriting a function

```rust
>fn inc x => x + 1
>a = 0
    0
>a = inc a
    1
>fn inc x => x + 2
>a = inc a
    3
```

\# Input:

Input will conform to either the `function` production or the `expression` production in the grammar below.

\# Output:

- Output for a valid function declaration will be an empty string (null in Java).
- Output for a valid expression will be the result of the expression.
- Output for input consisting entirely of whitespace will be an empty string (null in Java).
- All other cases will throw an error.

```rust
// In Rust that is:
Ok(None)
Ok(Some(f32))
Ok(None)
Err(String)
```

\# Language:

\# Grammar:

This section specifies the grammar for the interpreter language in [EBNF syntax](http://en.wikipedia.org/wiki/Extended_Backus%E2%80%93Naur_Form)

```rust
function        ::= fn-keyword fn-name { identifier } fn-operator expression
fn-name         ::= identifier
fn-operator     ::= '=>'
fn-keyword      ::= 'fn'

expression      ::= factor | expression operator expression
factor          ::= number | identifier | assignment | '(' expression ')' | function-call
assignment      ::= identifier '=' expression
function-call   ::= fn-name { expression }

operator        ::= '+' | '-' | '*' | '/' | '%'

identifier      ::= letter | '_' { identifier-char }
identifier-char ::= '_' | letter | digit

number          ::= { digit } [ '.' digit { digit } ]

letter          ::= 'a' | 'b' | ... | 'y' | 'z' | 'A' | 'B' | ... | 'Y' | 'Z'
digit           ::= '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9'
```

\# Operator Precedence:

The following table lists the language's operators grouped in order of precedence. Operators within each group have equal precedence.

| Category       | Operators |
| -------------- | --------- |
| Multiplicative | *, /, %   |
| Additive       | +, -      |
| Assignment     | =         |
| Function       | =>        |

## Thinking

