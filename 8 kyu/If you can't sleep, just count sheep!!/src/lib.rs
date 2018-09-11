fn countSheep(n: u32) -> String {
    (1..=n).map(|i| format!("{} sheep...", i)).collect()
}

#[test]
fn returns_expected() {
    assert_eq!(countSheep(1), "1 sheep...");
    assert_eq!(countSheep(2), "1 sheep...2 sheep...");
    assert_eq!(countSheep(3), "1 sheep...2 sheep...3 sheep...");
}
