fn hor_mirror(s: String) -> String { s.split('\n').rev().collect::<Vec<&str>>().join("\n") }

fn vert_mirror(s: String) -> String { s.split('\n').map(|s| s.chars().rev().collect::<String>()).collect::<Vec<String>>().join("\n") }

fn oper(oper: fn(String) -> String, s: String) -> String { oper(s) }
