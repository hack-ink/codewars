fn len_curve(n: i32) -> f64 {
    let mut v = vec![];
    for i in 0..n + 1 {
        let x = i as f64 / n as f64;
        let y = x.powi(2);
        v.push((x, y));
    }
    (1..n as usize + 1).fold(0., |acc, i| {
        let (x1, y1) = v[i - 1];
        let (x2, y2) = v[i];
        acc + ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    })
}
