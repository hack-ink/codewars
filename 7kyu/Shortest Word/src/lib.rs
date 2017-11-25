fn find_short(s: &str) -> usize { s.split_whitespace().map(str::len).min().unwrap() }
