fn is_prime(x: u64) -> bool {
    match x {
        2 | 3 => return true,
        x if x % 6 != 1 && x % 6 != 5 => return false,
        _ => (),
    }
    let mut i = 5;
    while i <= (x as f64).sqrt() as u64 {
        if x % i == 0 || x % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn gap(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let g = g as u64;
    let primes: Vec<u64> = (m..n + 1).filter(|&x| is_prime(x)).collect();
    if primes.len() > 1 {
        for i in 1..primes.len() {
            if primes[i] - primes[i - 1] == g { return Some((primes[i - 1], primes[i])); }
        }
    }
    None
}

fn testing(g: i32, m: u64, n: u64, exp: Option<(u64, u64)>) -> () {
    assert_eq!(gap(g, m, n), exp)
}

#[test]
fn basics_gap() {
    testing(2, 100, 110, Some((101, 103)));
    testing(4, 100, 110, Some((103, 107)));
    testing(6, 100, 110, None);
    testing(8, 300, 400, Some((359, 367)));
}