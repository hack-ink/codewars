extern crate num;
use num::complex::Complex;
 
fn f(z: Complex<f64>, eps: f64) -> i32 {
    if z.norm() < 1.0 { (eps.ln() / z.norm().ln()) as i32 } else { -1 }
}

#[cfg(test)]
    mod tests {
    use super::*;

    fn dotest(z: Complex<f64>, eps: f64, exp: i32) -> () {
        println!("z {:?}", z);
        println!("eps {:e}", eps);
        let ans = f(z, eps);
        let e = (ans - exp).abs() <= 1;
        println!("Actual {:?}", ans);
        println!("Expect {:?}", exp);
        println!("{:?}\n", e == true);
        assert_eq!(e, true);
    }

    #[test]
    fn basic_tests() {
        dotest(Complex::new(0.64, 0.75), 1e-12, 1952);
        dotest(Complex::new(30.0, 50.0), 1e-4, -1);
        dotest(Complex::new(0.3, 0.5), 1e-4, 17);
        dotest(Complex::new(0.13, 0.54), 1e-4, 15);

    }
}
