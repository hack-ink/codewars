fn diag_1_sym(s: &str) -> String {
    let mut result = vec![];
    let mut s = s.lines();
    while let Some(line) = s.next() { if result.is_empty() { for c in line.chars() { result.push(c.to_string()); } } else { for (i, c) in line.chars().enumerate() { result[i].push(c); } } }
    result.join("\n")
}

fn rot_90_clock(s: &str) -> String { diag_1_sym(s).lines().map(|line| { line.chars().rev().collect() }).collect::<Vec<String>>().join("\n") }

fn selfie_and_diag1(s: &str) -> String {
    let d = diag_1_sym(s);
    let mut d = d.lines();
    let mut s = s.lines();
    let mut result = vec![];
    while let Some(next) = s.next() { result.push(format!("{}|{}", next, d.next().unwrap())); }
    result.join("\n")
}

fn oper(f: fn(&str) -> String, s: &str) -> String { f(s) }