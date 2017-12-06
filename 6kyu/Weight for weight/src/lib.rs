fn order_weight(s: &str) -> String {
    let mut v: Vec<(u32, String)> = s.split_whitespace()
        .map(|x| {
            (
                x.chars().map(|c| c.to_digit(10).unwrap()).sum(),
                x.to_string(),
            )
        })
        .collect();
    v.sort();
    v.into_iter()
        .map(|(_, s)| s)
        .collect::<Vec<String>>()
        .join(" ")
}
