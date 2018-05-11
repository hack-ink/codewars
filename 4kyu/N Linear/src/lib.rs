fn n_linear(m: &[u32], n: usize) -> u32 {

}

#[test]
fn pair_test() {
    assert_eq!(n_linear(&[2, 3], 10), 22);
    assert_eq!(n_linear(&[3, 2], 10), 22);
    assert_eq!(n_linear(&[3, 2], 234), 1339);
}

#[test]
fn triplet_test() {
    assert_eq!(n_linear(&[5, 7, 8], 10), 64);
    assert_eq!(n_linear(&[5, 7, 8], 11), 65);
}
