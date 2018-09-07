## Detail

[Cafeteria](https://www.codewars.com/kata/cafeteria)

You have a simple Cafeteria.

You can create 3 coffee recipes:

-   Black = `Black coffee`
-   Cubano = `Cubano coffee` + `Brown sugar`
-   Americano = `Americano coffee` + `Milk with 0.5% fat`

And you can add a lot of extra sugar and extra milk in any coffee, e.g.:

-   Black + `Milk with 3.2% fat` + `Brown sugar`
-   Cubano + `Brown sugar` + `Brown sugar` + `Regular sugar`
-   Americano + `Milk with 3.2% fat` + `Milk with 0% fat` + `Regular sugar`

You need to create a `Coffee` by implementing a `CoffeeBuilder` struct like in `Builder` design pattern. 

This code is preloaded. You need only implement a `CoffeeBuilder` struct.

```rust
#[derive(Debug)]
struct Coffee {
    sort: String,
    milk: Vec<Milk>,
    sugar: Vec<Sugar>,
}

#[derive(Debug)]
struct Milk {
    fat: f32,
}

#[derive(Debug)]
struct Sugar {
    sort: String,
}

struct CoffeeBuilder {
    sort: String,
    milk: Vec<Milk>,
    sugar: Vec<Sugar>,
}
```

See test cases for more information and good luck!

## Thinking

Nothing to say… If you have any question then see [this](https://doc.rust-lang.org/book/second-edition/ch05-01-defining-structs.html).