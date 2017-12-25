fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn an(n: i64) -> Vec<i64> {
    let mut v = vec![7];
    while v.len() < n as usize {
        let prev = v.last().unwrap().clone();
        v.push(prev + gcd(n, prev));
    }
    v
}

fn gn(n: i64) -> Vec<i64> {
    let mut gn = an(n);
    if gn.len() > 1 {
        for in {}
    }
    gn
}

fn count_ones(n: i64) -> i64 {
    gn(n).into_iter().filter(|&x| x == 1).count() as i64
}

fn max_pn(n: i64) -> i64 {
    unimplemented!()
}

fn an_over_average(n: i64) -> i64 {
    unimplemented!()
}

fn testing1(n: i64, exp: i64) -> () {
    assert_eq!(count_ones(n), exp)
}

fn testing2(n: i64, exp: i64) -> () {
    assert_eq!(max_pn(n), exp)
}

fn testing3(n: i64, exp: i64) -> () {
    assert_eq!(an_over_average(n), exp)
}

#[test]
fn returns_expected() {
    testing1(1, 1);
    testing1(10, 8);
    testing1(100, 90);

//    testing2(1, 5);
//    testing2(5, 47);
//    testing2(7, 101);

//    testing3(5, 3);
}