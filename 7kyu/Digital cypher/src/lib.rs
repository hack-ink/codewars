fn encode(msg: String, n: i32) -> Vec<i32> {
    msg.chars()
        .zip(n.to_string().chars().cycle())
        .map(|(a, b)| a as i32 + b as i32 - 144)
        .collect()
}

#[test]
fn fixed_tests() {
    assert_eq!(encode("scout".to_string(), 1939), vec![20, 12, 18, 30, 21]);
    assert_eq!(encode("masterpiece".to_string(), 1939), vec![14, 10, 22, 29, 6, 27, 19, 18, 6, 12, 8]);
}