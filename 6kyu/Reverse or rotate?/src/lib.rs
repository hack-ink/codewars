fn revrot(s: &str, c: usize) -> String {
    if c <= 0 || s.is_empty() || c > s.len() { return String::from(""); }
    let v = s.chars().collect::<Vec<char>>();
    let mut chunks = v.chunks(c).filter(|chunk| chunk.len() == c);
    let mut result = String::new();
    while let Some(chunk) = chunks.next() {
        if chunk.iter().fold(0, |acc, x| acc + (x.to_digit(10).unwrap() as f64).powi(3) as i32) % 2 == 0 {
            result += &chunk.iter().rev().collect::<String>();
        } else {
            result += {
                let (head, tail) = chunk.split_at(1);
                &(tail.iter().collect::<String>() + &head.iter().collect::<String>())
            }
        }
    }
    result
}
