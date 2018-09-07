fn cycle(n: i64) -> i64 {
    if n % 5 == 0 || n % 2 == 0 {
        return -1;
    }
    let mut count = 0;
    let mut div = 1;
    loop {
        div *= 10 % n;
        count += 1;
        if div == 1 {
            return count;
        }
    }
}
