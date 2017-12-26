fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn an(n: i64) -> Vec<i64> {
    let mut v = vec![7];
    for i in 2..n + 1 {
        let prev = v.last().unwrap().clone();
        v.push(prev + gcd(i, prev));
    }
    v
}

fn gn(n: i64) -> Vec<i64> {
    let an = an(n);
    let mut gn = vec![1];
    for i in 1..an.len() {
        gn.push(an[i] - an[i - 1]);
    }
    gn
}

fn pn(n: i64) -> Vec<i64> {
    let mut pn: Vec<i64> = gn(n);
    pn.dedup();
    pn.sort();
    pn.dedup();
    pn
}

fn count_ones(n: i64) -> i64 {
    gn(n).into_iter().filter(|&x| x == 1).count() as i64
}

fn max_pn(n: i64) -> i64 {
    pn(n).last().unwrap().clone()
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

    testing2(1, 5);
    testing2(5, 47);
    testing2(7, 101);

//    testing3(5, 3);
}