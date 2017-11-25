fn reverse_words(str: &str) -> String { str.split_whitespace().rev().collect::<Vec<&str>>().join(" ") }
