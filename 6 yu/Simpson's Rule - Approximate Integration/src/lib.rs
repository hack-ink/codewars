use std::f64::consts::PI;

fn simpson(n: i32) -> f64 {
    let a = (1..((n / 2) + 1)).fold(0., |sum, i| sum + f((2 * i - 1) as f64 * h(n)));
    let b = (1..((n / 2))).fold(0., |sum, i| sum + f((2 * i) as f64 * h(n)));
    (PI / (3 * n) as f64) * (4. * a + 2. * b)
}

fn f(arg: f64) -> f64 {
    3. / 8. * ((3. * arg.sin()) - ((3. * arg).sin()))
}

fn h(n: i32) -> f64 {
    PI / n as f64
}
