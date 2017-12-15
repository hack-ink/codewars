/*fn init_prime(m: u64, n: u64) -> Vec<u64> {
    let mut check = vec![];
    let mut prime = vec![];
    check.resize(n as usize + 1, true);
    for i in 2..n + 1 {
        if check[i as usize] {
            prime.push(i);
        }
        for j in 0..prime.len() {
            if i * prime[j] > n {
                break;
            }
            check[(i * prime[j]) as usize] = false;
            if i % prime[j] == 0 {
                break;
            }
        }
    }
    prime.into_iter().filter(|&x| x >= m).collect()
}*/

/*fn is_prime(x: u64) -> bool {
    if x < 2 {
        return false;
    }
    let mut n = 2;
    while n * n <= x {
        if x % n == 0 {
            return false;
        }
        n += 1;
    }
    true
}*/

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

fn step(g: i32, m: u64, n: u64) -> Option<(u64, u64)> {
    let mut m = m;
    while m <= n - g as u64 {
        if is_prime(m) && is_prime(m + g as u64) {
            return Some((m, m + g as u64));
        }
        m += 1;
    }
    None
}
