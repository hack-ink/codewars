fn countSheep(n: u32) -> String {
    let mut s = String::new();
    for i in 1..=n {
        s = format!("{}{} sheep...", s, i);
    }

    s
}

#[test]
fn returns_expected() {
    assert_eq!(countSheep(1), "1 sheep...");
    assert_eq!(countSheep(2), "1 sheep...2 sheep...");
    assert_eq!(countSheep(3), "1 sheep...2 sheep...3 sheep...");
}