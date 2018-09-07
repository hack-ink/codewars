fn going(n: i32) -> f64 {
    let mut res: f64 = 1.0;
    let mut inter: f64 = 1.0;
    let mut i: i32 = n;
    while i >= 2 {
        inter = inter * (1.0 / i as f64);
        res += inter;
        i -= 1
    }
    (res * 1e6).floor() / 1e6
}
