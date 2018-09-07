fn diagonal(n: u32, p: u32) -> u64 {
    let mut a: u64 = 1;
    (1..((n - p + 1) as u64)).fold(1, |acc, j| {
          a = a * (j + p as u64) / j;
          acc + a
        }
    )
}
