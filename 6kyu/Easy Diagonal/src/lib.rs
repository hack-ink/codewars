fn diagonal(n: u32, p: u32) -> u64 {
    let mut f = 1_f64;
    let mut d = 1_f64;
    for i in 0..(1 + p) {
        f *= n as f64 + 1.0 - i as f64;
        d *= i as f64 + 1.0;
    }
    (f / d).round() as u64
}
