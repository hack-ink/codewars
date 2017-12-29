//use std::collections::VecDeque;
//
//fn dbl_linear(n: u32) -> u32 {
//    let mut x = 1;
//    let mut q2 = VecDeque::new();
//    let mut q3 = VecDeque::new();
//    for _ in 0..n {
//        q2.push_back(2 * x + 1);
//        q3.push_back(3 * x + 1);
//        x = std::cmp::min(q2[0], q3[0]);
//        if x == q2[0] { x = q2.pop_front().unwrap(); }
//        if x == q3[0] { x = q3.pop_front().unwrap(); }
//    }
//    x
//}

fn dbl_linear(n: u32) -> u32 {
    let mut x = 1;
    let mut y = vec![];
    let mut z = vec![];
    for _ in 0..n {
        let y_ = 2 * x + 1;
        let z_ = 3 * x + 1;
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