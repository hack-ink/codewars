fn valid_braces(s: &str) -> bool {
    false
}

#[test]
fn basic_tests() {
    assert!(valid_braces("()"));
    assert!(!valid_braces("[(])"));
}