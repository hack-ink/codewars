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

impl CoffeeBuilder {
    fn new() -> CoffeeBuilder {
        unimplemented!()
    }

    fn set_black_coffee(mut self) -> CoffeeBuilder {
        unimplemented!()
    }

    fn set_cubano_coffee(mut self) -> CoffeeBuilder {
        unimplemented!()
    }

    fn set_antoccino_coffee(mut self) -> CoffeeBuilder {
        unimplemented!()
    }

    fn with_milk(mut self, fat: f32) -> CoffeeBuilder {
        unimplemented!()
    }

    fn with_sugar(mut self, sort: String) -> CoffeeBuilder {
        unimplemented!()
    }

    fn build(self) -> Coffee {
        unimplemented!()
    }
}

#[test]
fn sample_tests() {
    let coffee = CoffeeBuilder::new()
        .set_black_coffee()
        .with_sugar("Regular".to_string())
        .with_milk(3.2)
        .build();
    assert_eq!(format!("{:?}", coffee), "Coffee { sort: \"Black\", milk: [Milk { fat: 3.2 }], sugar: [Sugar { sort: \"Regular\" }] }");

    let coffee = CoffeeBuilder::new()
        .set_antoccino_coffee()
        .build();
    assert_eq!(format!("{:?}", coffee), "Coffee { sort: \"Americano\", milk: [Milk { fat: 0.5 }], sugar: [] }");

    let coffee = CoffeeBuilder::new()
        .set_cubano_coffee()
        .build();
    assert_eq!(format!("{:?}", coffee), "Coffee { sort: \"Cubano\", milk: [], sugar: [Sugar { sort: \"Brown\" }] }");
}