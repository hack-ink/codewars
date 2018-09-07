use std::cmp;

fn prod2sum(a: i64, b: i64, c: i64, d: i64) -> Vec<(i64, i64)> {
    let mut res = vec![];
    let e1 = (a * c + b * d).abs();
    let f1 = (a * d - b * c).abs();
    let e2 = (a * c - b * d).abs();
    let f2 = (a * d + b * c).abs();
    if (e1 == f2 && f1 == e2) || (e1 == e2 && f1 == f2) {
        res.push((cmp::min(e1, f1), cmp::max(e1, f1)));
    } else {
        res.push((cmp::min(e1, f1), cmp::max(e1, f1)));
        res.push((cmp::min(e2, f2), cmp::max(e2, f2)));
        res.sort();
    }
    res
}