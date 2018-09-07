fn finance(n: u64) -> u64 {
    n * (n + 1) * (n + 2) / 2
}

/*fn finance(n: u64) -> u64 {
    (0..n + 1)
        .fold((0, 0), |(an, sum), x| {
            let an = an + 3 * x;
            (an, sum + an)
        })
        .1
}*/
