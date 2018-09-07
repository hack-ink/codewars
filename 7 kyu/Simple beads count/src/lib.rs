fn count_red_beads(n: u32) -> u32 {
    match n {
        0 | 1 => 0,
        _ => (n - 1) * 2
    }
}

#[test]
fn test() {
    assert_eq!(count_red_beads(0), 0);
    assert_eq!(count_red_beads(1), 0);
    assert_eq!(count_red_beads(3), 4);
    assert_eq!(count_red_beads(5), 8);
}