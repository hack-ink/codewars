fn temps(v0: i32, slope: i32, d_tot: i32) -> i32 {
}

fn dotest(v0: i32, slope: i32, d_tot: i32, exp: i32) -> () {
    assert_eq!(exp, temps(v0, slope, d_tot))
}

#[test]
fn basic_tests() {
    dotest(30, 5, 30, 114);
    dotest(30, 20, 30, -1);
    dotest(30, 8, 20, 110);
    dotest(30, 0, 5, 9);
    dotest(50, 10, 25, 185);
}
