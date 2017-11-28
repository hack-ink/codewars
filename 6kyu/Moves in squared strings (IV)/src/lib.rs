fn diag_2_sym(s: &str) -> String {
    let mut v = vec![];
    let mut lines = s.lines().rev();
    for c in lines.next().unwrap().chars() {
        v.push(c.to_string());
    }
    while let Some(line) = lines.next() {
        for (i, c) in line.chars().enumerate() {
            v[i].push(c);
        }
    }
    v.reverse();
    v.join("\n")
}

fn rot_90_counter(s: &str) -> String {
    diag_2_sym(s)
        .lines()
        .map(|line| line.chars().rev().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

fn selfie_diag2_counterclock(s: &str) -> String {
    let mut v = vec![];
    for line in s.lines() {
        v.push(line.to_string());
    }
    append(&mut v, diag_2_sym(s));
    append(&mut v, rot_90_counter(s));
    v.join("\n")
}

fn append(v: &mut Vec<String>, s: String) {
    let lines = s.lines();
    for (i, line) in lines.enumerate() {
        v[i].push_str(format!("|{}", line).as_str());
    }
}

fn oper(f: fn(&str) -> String, s: &str) -> String {
    f(s)
}
