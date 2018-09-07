//#![feature(repeat_str)] <--since repeat_str is not stable on 1.10 used by codewars we have to do it wit iter::repeat
fn repeat(s: &str, n: usize) -> String { std::iter::repeat(s).take(n).collect() }

fn rot(s: &str) -> String { s.chars().rev().collect() }

fn selfie_and_rot(s: &str) -> String { s.split('\n').map(|s| s.to_string() + &repeat(".", s.chars().count())).collect::<Vec<String>>().join("\n") + "\n" + &rot(s).split('\n').map(|s| repeat(".", s.chars().count()) + s).collect::<Vec<String>>().join("\n") }

fn oper(oper: fn(&str) -> String, s: &str) -> String { oper(s) }
