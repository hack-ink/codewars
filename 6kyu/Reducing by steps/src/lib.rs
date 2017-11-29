use std::cmp;

fn som(x: i64, y: i64) -> i64 {
    x + y
}

fn maxi(x: i64, y: i64) -> i64 {
    cmp::max(x, y)
}

fn mini(x: i64, y: i64) -> i64 {
    cmp::min(x, y)
}

fn gcdi(a: i64, b: i64) -> i64 {
    let a = a.abs();
    let b = b.abs();
    match b {
        0 => a,
        _ => gcdi(b, a % b),
    }
}

fn lcmu(a: i64, b: i64) -> i64 {
    (a.abs() * b.abs()) / gcdi(a, b)
}

fn oper_array(f: fn(i64, i64) -> i64, a: &[i64], init: i64) -> Vec<i64> {
    a.iter().fold(vec![init], |mut acc, &x| {
        let last = acc.last().unwrap().clone();
        acc.push(f(last, x));
        acc
    })
        [1..]
        .to_vec()
}
