fn to_alternating_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() { if c.is_uppercase() { result.extend(c.to_lowercase()) } else { result.extend(c.to_uppercase()) } }
    result
}