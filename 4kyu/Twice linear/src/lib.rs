fn dbl_linear(n: u32) -> u32 {
    let mut n = n;
    let mut v = vec![1];
    while let Some(&last) = v.last() {
        let y = last * 2 + 1;
        let z = y + last;
        v.push(y);
        v.push(z);
        n -= 1;
        if n == 0 { break; }
        println!("{:?}", v);
    }
    v.sort();
    v.dedup();
    return *v.last().unwrap();
}

fn testing(n: u32, exp: u32) -> () {
    assert_eq!(dbl_linear(n), exp)
}

#[test]
fn basics_dbl_linear() {
    testing(10, 22);
    testing(20, 57);
    testing(30, 91);
    testing(50, 175);
    testing(100, 447);
}