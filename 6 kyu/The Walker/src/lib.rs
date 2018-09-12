fn solve(a: i32, b: i32, c: i32, alpha: i32, beta: i32, gamma: i32) -> Vec<i32> {
    let (a, b, c, alpha, beta, gamma) = (
        a as f64,
        b as f64,
        c as f64,
        (alpha as f64).to_radians(),
        (beta as f64).to_radians(),
        (gamma as f64).to_radians(),
    );

    let dt = a * alpha.sin();
    let ot = a * alpha.cos();
    let be = b * beta.sin();
    let de = b * beta.cos();
    let cf = c * gamma.sin();
    let bf = c * gamma.cos();

    let et = de + dt;
    let ef = bf + be;
    let cg = et - cf;
    let go = ef - ot;

    let co = (cg.powi(2) + go.powi(2)).sqrt();

    let sin_cog = cg / co;
    let arcsin_cog = sin_cog.asin() / std::f64::consts::PI * 180.;

    let cot_degree = 180. - arcsin_cog;
    let cot_minute = (cot_degree - cot_degree.trunc()) * 60.;
    let cot_second = (cot_minute - cot_minute.trunc()) * 60.;

    vec![co.round() as i32, cot_degree as i32, cot_minute as i32, cot_second as i32]
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(a: i32, b: i32, c: i32, aa: i32, bb: i32, cc: i32, exp: Vec<i32>) -> () {
        let ans = solve(a, b, c, aa, bb, cc);
        println!("{:?}", ans == exp);
        assert_eq!(ans, exp)
    }

    #[test]
    fn basic_tests() {
        dotest(12, 20, 18, 45, 30, 60, vec![15, 135, 49, 18]);
        dotest(15, 15, 19, 50, 29, 55, vec![12, 133, 18, 44]);
        dotest(14, 25, 17, 41, 35, 59, vec![20, 129, 41, 57]);
    }
}