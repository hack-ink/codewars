fn order(sentence: &str) -> String {
    let mut ws: Vec<_> = sentence.split_whitespace().map(String::from).collect();
    ws.sort_by_key(|s| s.chars().find(|c| c.is_digit(10)).unwrap());
    ws.join(" ")
}

/*fn order(sentence: &str) -> String {
    let mut words: Vec<String> = sentence
        .split_whitespace()
        .map(|word| word.to_string())
        .collect();
    words.sort_by_key(|word| word.chars().find(|c| c.is_numeric()));
    words.join(" ")
}*/
