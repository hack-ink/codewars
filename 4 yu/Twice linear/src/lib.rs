//use std::collections::VecDeque;
//
//fn dbl_linear(n: u32) -> u32 {
//    let mut x = 1;
//    let mut y = VecDeque::new();
//    let mut z = VecDeque::new();
//    for _ in 0..n {
//        let y_ = 2 * x + 1;
//        let z_ = y_ + x;
//        y.push_back(y_);
//        z.push_back(z_);
//        x = std::cmp::min(y[0], z[0]);
//        if x == y[0] { x = y.pop_front().unwrap(); }
//        if x == z[0] { x = z.pop_front().unwrap(); }
//    }
//    x
//}

fn dbl_linear(n: u32) -> u32 {
    let mut x = 1;
    let mut y = vec![];
    let mut z = vec![];
    for _ in 0..n {
        let y_ = 2 * x + 1;
        let z_ = y_ + x;
        y.push(y_);
        z.push(z_);
        x = std::cmp::min(y[0], z[0]);
        if x == y[0] { x = y.remove(0); }
        if x == z[0] { x = z.remove(0); }
    }
    x
}

fn testing(n: u32, exp: u32) -> () {
    assert_eq!(dbl_linear(n), exp)
}

#[test]
fn basics_dbl_linear() {
    testing(10, 22);
//    testing(20, 57);
//    testing(30, 91);
//    testing(50, 175);
//    testing(100, 447);
}
