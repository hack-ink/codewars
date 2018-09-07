fn solution(n: f64) -> f64 {
    (2.0 * n).round() / 2.0
}

/*fn solution(n: f64) -> f64 {
    match n - n.trunc() {
        x if x < 0.25 => n.trunc(),
        x if x < 0.75 => n.trunc() + 0.5,
        _ => n.ceil(),
    }
}*/
