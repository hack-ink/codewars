fn solve(m: f64) -> f64 {
    ((2. * m + 1.) - (4. * m + 1.).sqrt()) / m / 2.
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_fuzzy(m: f64, expect: f64) -> () {
        let merr = 1e-12;
        println!("{:?}", m);
        let actual = solve(m);
        println!("Actual {:e}", actual);
        println!("Expect {:e}", expect);
        let inrange = (actual - expect).abs() <= merr;
        if inrange == false {
            println!("Expected value near: {:e} but got: {:e}", actual, expect);
        }
        assert_eq!(inrange, true);
    }

    #[test]
    fn basic_tests() {
        assert_fuzzy(2.00, 5.000000000000e-01);
        assert_fuzzy(4.00, 6.096117967978e-01);
        assert_fuzzy(5.00, 6.417424305044e-01);
    }
}
