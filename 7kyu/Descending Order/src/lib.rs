fn descending_order(x: u64) -> u64 {
    let mut v: Vec<u32> = x.to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect();
    v.sort();
    v.into_iter().rev().fold(0, |acc, n| acc * 10 + n as u64)
}
