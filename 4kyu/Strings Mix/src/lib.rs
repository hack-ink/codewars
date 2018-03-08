use std::cmp::Ordering;

const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn mix(s1: &str, s2: &str) -> String {
    let mut weighted_chars = ALPHABET.chars().filter_map(|c| {
        let c1 = s1.chars().filter(|&x| x == c).count();
        let c2 = s2.chars().filter(|&x| x == c).count();
        if c1 <= 1 && c2 <= 1 { return None; }
        if c1 > c2 { Some(format!("1:{}", &std::iter::repeat(c.to_string()).take(c1 as usize).collect::<String>())) } else if c2 > c1 { Some(format!("2:{}", &std::iter::repeat(c.to_string()).take(c2 as usize).collect::<String>())) } else { Some(format!("=:{}", &std::iter::repeat(c.to_string()).take(c1 as usize).collect::<String>())) }
    }).collect::<Vec<String>>();
    weighted_chars.sort_by(|a, b| {
        match a.len().cmp(&b.len()) {
            Ordering::Less => { Ordering::Greater }
            Ordering::Greater => { Ordering::Less }
            Ordering::Equal => { a.cmp(&b) }
        }
    });
    weighted_chars.join("/")
}
