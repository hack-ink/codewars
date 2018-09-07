fn doubles(maxk: i32, maxn: i32) -> f64 {
    (1..maxk + 1).map(|k| (1..maxn + 1).map(|x| ((x + 1) as f64).powi(-2 * k)).sum::<f64>() / k as f64).sum()
}
