fn check_choose(m: u64, n: u64) -> i64 {
    let mut result = 1;
    for i in 0..n + 1 {
        if result == m {
            return i as i64;
        };
        result = result * (n - i) / (i + 1);
    }
    -1
}
