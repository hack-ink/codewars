fn gcd(a: i64, b: i64) -> i64 {
    let mut m = a;
    let mut n = b;
    if m == 0 || n == 0 {
        return m | n;
    }
    let shift = (m | n).trailing_zeros();
    n >>= n.trailing_zeros();
    while m != 0 {
        m >>= m.trailing_zeros();
        if n > m {
            ::std::mem::swap(&mut n, &mut m)
        }
        m -= n;
    }
    n << shift
}

fn from_nb_2str(n: i64, sys: Vec<i64>) -> String {
    if sys.iter().fold(1, |acc, x| acc * x) < n {
        return "Not applicable".to_string();
    }
    for &a in sys.iter() {
        for &b in sys.iter() {
            if a == b {
                continue;
            }
            if gcd(a, b) != 1 {
                return "Not applicable".to_string();
            }
        }
    }
    sys.into_iter().fold(
        String::new(),
        |acc, x| acc + format!("-{}-", n % x).as_str(),
    )
}
