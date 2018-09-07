fn epidemic(tm: i32, n: i32, s0: i32, i0: i32, b: f64, a: f64) -> i32 {
    let dt = tm as f64 / n as f64;
    let mut r = 0.0;
    let mut s = s0 as f64;
    let mut i = i0 as f64;
    let mut max = 0.0;
    for _ in 0..n {
        let st = s - dt * b * s * i;
        let it = i + dt * (b * s * i - a * i);
        let rt = r + dt * i * a;
        r = rt;
        i = it;
        s = st;
        if max < i {
            max = i;
        }
    }
    max as i32
}
