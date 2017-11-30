fn rnd10(f: f64) -> f64 {
    (f * 1e10).round() / 1e10
}

fn iter_pi(epsilon: f64) -> (i32, f64) {
    let mut e = (1., 0., 0);
    while (std::f64::consts::PI - 4. * e.1).abs() > epsilon {
        e.1 += 1. / e.0;
        e.0 = -e.0;
        if e.0 > 0. {
            e.0 += 2.;
        }
        if e.0 < 0. {
            e.0 -= 2.;
        }
        e.2 += 1;
    }
    (e.2, rnd10(e.1 * 4.))
}
