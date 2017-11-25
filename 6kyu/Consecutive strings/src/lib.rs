fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    let mut result = String::new();
    if k > 0 && strarr.len() >= k {
        for index in 0..strarr.len() - k + 1 {
            let s: String = strarr[index..index + k].join("");
            if s.len() > result.len() { result = s; }
        }
    }
    result
}
