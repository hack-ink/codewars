use std::cmp::*;

fn how_much(m: i32, n: i32) -> Vec<(String, String, String)> {
    (min(m, n)..max(m, n) + 1).fold(vec![], |mut acc, x| {
        if x % 9 == 1 && x % 7 == 2 {
            acc.push((
                format!("{}: {}", "M", x),
                format!("{}: {}", "B", x / 7),
                format!("{}: {}", "C", x / 9),
            ));
        }
        acc
    })
}
