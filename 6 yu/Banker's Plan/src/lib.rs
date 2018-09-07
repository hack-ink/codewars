fn fortune(f0: i32, p: f64, c0: i32, n: i32, i: f64) -> bool {
    let mut f = f0 as f64;
    let mut c = c0 as f64;
    let p = p / 100. + 1.;
    let i = i / 100. + 1.;
    for _ in 0..n {
        f = (f * p - c).trunc();
        c = (c * i).trunc();
        if f < 0. {
            return false;
        }
    }
    true
}
