fn perimeter(mut n: u64) -> u64 {
    (0..n)
        .fold((1u64, 1u64, 1u64), |a, _| (a.1, a.0 + a.1, a.1 + a.2))
        .2 * 4
}

/*fn perimeter(n: u64) -> u64 {
    let mut n = n;
    match n {
        0 | 1 => return n,
        _ => {
            let mut a = n & 1;
            let mut b = 1;
            let mut sum = a + b;
            n /= 2;
            while n > 0 {
                a += b;
                b += a;
                sum += a + b;
                n -= 1;
            }
            return sum * 4;
        }
    }
}*/
