fn max_number(n: u32) -> u32 {
    let mut digits = n.to_string().as_bytes().to_vec();
    digits.sort_by(|a, b| b.cmp(a));

    String::from_utf8(digits).unwrap().parse::<u32>().unwrap()
}

#[test]
fn basic_tests() {
    assert_eq!(max_number(213), 321);
    assert_eq!(max_number(7389), 9873);
    assert_eq!(max_number(63729), 97632);
    assert_eq!(max_number(566797), 977665);
    assert_eq!(max_number(17693284), 98764321);
}
