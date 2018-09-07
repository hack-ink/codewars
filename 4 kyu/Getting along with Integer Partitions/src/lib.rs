fn part(n: i64) -> String {
    let mut v = Vec::new();
    let mut p = Vec::new();
    p.resize(n as usize, 0i64);
    partition(n, n, 0, &mut v, &mut p);
    let mut v: Vec<i64> = v.into_iter().map(|x| x.iter().fold(1, |acc, x| acc * x)).collect();
    v.sort();
    v.dedup();
    let len = v.len();
    let index = len / 2;
    let median;
    if len % 2 == 0 { median = (v[index - 1] + v[index]) as f64 / 2f64; } else { median = v[index] as f64; }
    format!("Range: {} Average: {:.2} Median: {:.2}", v.last().unwrap() - v.first().unwrap(), v.iter().sum::<i64>() as f64 / len as f64, median)
}

fn partition(n: i64, k: i64, index: usize, v: &mut Vec<Vec<i64>>, p: &mut Vec<i64>) {
    if n == 0 {
        let mut p = p.clone();
        p.truncate(index);
        v.push(p.into_iter().filter(|&x| x != 1).collect::<Vec<i64>>());
        return;
    }
    for i in (1..k + 1).rev() {
        if i > n { continue; }
        p[index] = i;
        partition(n - i, i, index + 1, v, p);
    }
}
