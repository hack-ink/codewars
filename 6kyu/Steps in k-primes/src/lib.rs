fn kprimes_step(k: i32, step: i32, m: u64, n: u64) -> Option<Vec<(u64, u64)>> {}

fn testing(k: i32, step: i32, m: u64, n: u64, exp: Option<Vec<(u64, u64)>>) -> () {
    let ans = kprimes_step(k, step, m, n);
    println!("{:?}", ans == exp);
    assert_eq!(kprimes_step(k, step, m, n), exp)
}

#[test]
fn basic_tests() {
    testing(10, 8, 2425364, 2425700, None);
    testing(6, 8, 2627371, 2627581, Some(vec![(2627408, 2627416), (2627440, 2627448), (2627496, 2627504)]));
}