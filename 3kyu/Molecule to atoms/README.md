## Detail

[Molecule to atoms](https://www.codewars.com/kata/52f831fa9d332c6591000511)

For a given chemical formula represented by a string, count the number of atoms of each element contained in the molecule and return an object (associative array in PHP, `Dictionary<string, int>` in C#, Map in Java).

For example:

```rust
parse_molecule("H2O");           // water
// Ok([("H", 2), ("O", 1)])

parse_molecule("Mg(OH)2");       // magnesium hydroxide
// Ok([("Mg", 1), ("O", 2), ("H", 2)]

parse_molecule("K4[ON(SO3)2]2"); // Fremy's salt
// Ok([("K", 4), ("O", 14),("N", 2),("S", 4)])

parse_molecule("pie")
// Err(ParseError)
```

As you can see, some formulas have brackets in them. The index outside the brackets tells you that you have to multiply count of each atom inside the bracket on this index. For example, in Fe(NO3)2 you have one iron atom, two nitrogen atoms and six oxygen atoms.

Note that brackets may be **round, square or curly** and can also be **nested**. Index after the braces is **optional**.

## Thinking

