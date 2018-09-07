fn game(n: u64) -> Vec<u64> {
    if n % 2 == 0 {
        vec![n * n / 2]
    } else {
        vec![n * n, 2]
    }
}
