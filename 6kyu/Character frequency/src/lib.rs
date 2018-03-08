use std::collections::BTreeMap;
use std::ascii::AsciiExt;

fn letter_frequency(input: &str) -> BTreeMap<char, i32> {
    let mut count: BTreeMap<char, i32> = BTreeMap::new();

    for x in input.chars().filter(|c| c.is_alphabetic()).map(|c| AsciiExt::to_ascii_lowercase(&c)) {
        *count.entry(x).or_insert(0) += 1;
    }

    count
}

#[test]
fn simpleword() {
    let answer: BTreeMap<char, i32> =
        [('a', 2),
            ('c', 1),
            ('l', 1),
            ('t', 1),
            ('u', 1)]
            .iter().cloned().collect();

    assert_eq!(letter_frequency("actual"), answer);
}

#[test]
fn sequence() {
    let answer: BTreeMap<char, i32> =
        [('a', 3),
            ('b', 2),
            ('f', 1),
            ('p', 1),
            ('s', 1),
            ('t', 2),
            ('u', 1),
            ('x', 5)]
            .iter().cloned().collect();

    assert_eq!(letter_frequency("AaabBF UttsP xxxxx"), answer);
}