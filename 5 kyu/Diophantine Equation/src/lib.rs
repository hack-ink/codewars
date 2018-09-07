fn solequa(n: u64) -> Vec<(u64, u64)> {
    let mut v = vec![];
    if n % 4 == 2 {
        return v;
    }
    let root = (n as f64).sqrt() as u64;
    for a in 1..root + 1 {
        let b = n / a;
        if b * a != n || (b - a) % 4 != 0 {
            continue;
        }
        let y = (b - a) / 4;
        let x = a + 2 * y;
        v.push((x, y));
    }
    v
}
