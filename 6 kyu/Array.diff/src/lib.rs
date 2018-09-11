fn array_diff<T: PartialEq>(mut a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.retain(|x| !b.contains(x));

    a
}

#[test]
fn returns_expected() {
    assert_eq!(array_diff(vec![1, 2], vec![1]), vec![2]);
    assert_eq!(array_diff(vec![1, 2, 2], vec![1]), vec![2, 2]);
    assert_eq!(array_diff(vec![1, 2, 2], vec![2]), vec![1]);
    assert_eq!(array_diff(vec![1, 2, 2], vec![]), vec![1, 2, 2]);
    assert_eq!(array_diff(vec![], vec![1, 2]), vec![]);
}
