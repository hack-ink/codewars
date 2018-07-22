fn remove_duplicate_words(s: &str) -> String {
    let mut v: Vec<&str> = vec![];

    for s in s.split_whitespace() { if !v.contains(&s) { v.push(s); } }

    v.join(" ")
}

#[test]
fn sample_test_cases() {
    assert_eq!(remove_duplicate_words("alpha beta beta gamma gamma gamma delta alpha beta beta gamma gamma gamma delta"), "alpha beta gamma delta");
    assert_eq!(remove_duplicate_words("my cat is my cat fat"), "my cat is fat");
}