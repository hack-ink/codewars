fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    // your code
}

fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
    assert_eq!(gap(g, m, n), exp)
}

#[test]
fn basics_gap() {
    testing(2,100,110, Some((101, 103)));
    testing(4,100,110, Some((103, 107)));
    testing(6,100,110, None);
    testing(8,300,400, Some((359, 367)));
}