fn max_multiple(divisor: u32, bound: u32) -> u32 {
    bound / divisor * divisor
}

#[test]
fn basic_test() {
    assert_eq!(max_multiple(2,7),6);
    assert_eq!(max_multiple(3,10),9);
    assert_eq!(max_multiple(7,17),14);
    assert_eq!(max_multiple(10,50),50);
}