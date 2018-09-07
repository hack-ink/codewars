fn dist(v: f64, mu: f64) -> f64 {
    let v = v / 3.6;
    v + v * v / (2. * mu * 9.81)
}

fn speed(d: f64, mu: f64) -> f64 {
    let b = -2. * mu * 9.81;
    3.6 * (b + (b * b - 4. * b * d).sqrt()) / 2.
}
