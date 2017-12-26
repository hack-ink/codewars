fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn count_ones(n: i64) -> i64 {
    if n == 0 { return 0; }
    let mut gn = vec![1];
    let mut an_prev = 7;
    for i in 2..n + 1 {
        let gcd = gcd(i, an_prev);
        if gcd == 1 { gn.push(gcd); }
        an_prev += gcd;
    }
    gn.len() as i64
}

fn max_pn(n: i64) -> i64 {
    if n == 0 { return 0; }
    let mut prev = 7;
    let mut pn = vec![];
    let mut i = 2;
    while pn.len() < n as usize {
        let next = prev + gcd(i, prev);
        let p = next - prev;
        prev = next;
        if p != 1 && !pn.contains(&p) { pn.push(p); }
        i += 1;
    }
    pn.into_iter().max().unwrap()
}

fn an_over_average(n: i64) -> i64 {
    if n == 0 { return 0; }
    let mut gn = 1;
    let mut an_prev = 7;
    let mut i = 1;
    let mut an_over = vec![];
    while (an_over.len() as i64) < n {
        if gn != 1 { an_over.push(an_prev / i); }
        i += 1;
        gn = gcd(i, an_prev);
        an_prev += gn;
    }
    an_over.into_iter().sum::<i64>() / n
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

    testing3(5, 3);
    testing3(1, 3);
}