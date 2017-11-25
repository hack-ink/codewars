fn arrange(s: &str) -> String {
    if s.len() == 0 { return String::from(""); }
    let mut v = s.split_whitespace().map(|s| s.to_uppercase()).collect::<Vec<String>>();
    let len = v.len() - 1;
    for i in 1..len + 1 {
        if i % 2 == 0 { if v[i - 1].len() < v[i].len() { v.swap(i - 1, i); } } else {
            if v[i - 1].len() > v[i].len() { v.swap(i - 1, i); }
            v[i - 1] = v[i - 1].to_lowercase();
        }
    }
    if len % 2 == 0 { v[len] = v[len].to_lowercase(); }
    v.join(" ")
}
