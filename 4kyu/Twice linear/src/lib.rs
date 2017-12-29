use std::collections::HashSet;
fn dbl_linear(n: u32) -> u32 {
    let mut v = vec![];
    for i in 0..11 {
        v.push(2 * i + 1);
        v.push(3 * i + 1);
    }
    v.sort();
    println!("{:?}", v);
    0
}

fn testing(n: u32, exp: u32) -> () {
    assert_eq!(dbl_linear(n), exp)
}

#[test]
fn basics_dbl_linear() {
//    testing(10, 22);
    testing(20, 57);
//    testing(30, 91);
//    testing(50, 175);
//    testing(100, 447);
}