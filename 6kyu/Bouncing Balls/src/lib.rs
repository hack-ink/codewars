fn bouncing_ball(h: f64, bounce: f64, window: f64) -> i32 {
    if h <= 0. || bounce <= 0. || bounce >= 1. || window >= h { return -1; }
    let mut h = h * bounce;
    let mut times = 1;
    while h > window {
        times += 2;
        h *= bounce;
    }
    times
}
