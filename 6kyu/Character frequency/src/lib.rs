use std::collections::BTreeMap;

fn letter_frequency(input: &str) -> BTreeMap<char, i32> {
    unimplemented!();
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