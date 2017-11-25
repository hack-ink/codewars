fn find_nb(n: u64) -> i32 {
    let kk = (4.0 * n as f64).sqrt().sqrt().floor() as u64;
    if 4u64 * n == kk * kk * (kk + 1u64) * (kk + 1u64) { kk as i32 } else { -1 }
}
