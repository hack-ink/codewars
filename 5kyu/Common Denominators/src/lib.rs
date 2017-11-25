fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    if l.is_empty() { return vec![]; }
    let l = l.iter().map(|&(a, b)| {
        let gcd = gcd(a, b);
        if gcd == 1 { (a, b) } else { (a / gcd, b / gcd) }
    }).collect::<Vec<(i64, i64)>>();
    let gcd = l.iter().fold(1, |acc, &(_, x)| lcm(acc, x));
    l.iter().map(|&(a, b)| { (gcd / b * a, gcd) }).collect()
}

//fn gcd(a: i64, b: i64) -> i64 {
//    let mut m = a;
//    let mut n = b;
//    while m != 0 {
//        let temp = m;
//        m = n % temp;
//        n = temp;
//    }
//    n
//}

fn gcd(a: i64, b: i64) -> i64 {
    // Use Stein's algorithm
    let mut m = a;
    let mut n = b;
    if m == 0 || n == 0 { return m | n; }
    // find common factors of 2
    let shift = (m | n).trailing_zeros();
    // divide n and m by 2 until odd
    // m inside loop
    n >>= n.trailing_zeros();
    while m != 0 {
        m >>= m.trailing_zeros();
        if n > m { ::std::mem::swap(&mut n, &mut m) }
        m -= n;
    }
    n << shift
}

fn lcm(a: i64, b: i64) -> i64 { (a * b) / gcd(a, b) }
