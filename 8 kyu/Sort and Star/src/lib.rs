fn two_sort(arr: &[&str]) -> String {
    arr.iter()
        .min()
        .unwrap()
        .chars()
        .fold(String::new(), |acc, s| format!("{}***{}", acc, s))[3..]
        .to_string()
}

#[test]
fn sample_test_cases() {
    assert_eq!(two_sort(&["bitcoin", "take", "over", "the", "world", "maybe", "who", "knows", "perhaps"]), "b***i***t***c***o***i***n");
    assert_eq!(two_sort(&["turns", "out", "random", "test", "cases", "are", "easier", "than", "writing", "out", "basic", "ones"]), "a***r***e");
}
