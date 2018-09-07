fn max_ball(v0: i32) -> i32 {
    (((1000 * v0) as f64 / 3.6 / 9.81) / 100.).round() as i32
}
