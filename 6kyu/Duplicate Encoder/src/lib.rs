fn duplicate_encode(word: &str) -> String {
    let lower = String::from(word).to_lowercase();
    lower
        .chars()
        .map(|c| if lower.find(c) == lower.rfind(c) {
            '('
        } else {
            ')'
        })
        .collect()
}

/*use std::ascii::AsciiExt;

fn duplicate_encode(word: &str) -> String {
    let word: String = word.chars().map(|c| c.to_ascii_lowercase()).collect();
    let mut a = vec![];
    let mut b = vec![];
    for c in word.chars() {
        if a.contains(&c) {
            if !b.contains(&c) {
                b.push(c);
            }
        } else {
            a.push(c);
        }
    }
    word.chars()
        .map(|c| if b.contains(&c) { ')' } else { '(' })
        .collect()
}*/
