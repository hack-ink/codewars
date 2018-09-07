fn list_squared(m: u64, n: u64) -> Vec<(u64, u64)> {
    (m..n + 1)
        .map(|x| (x, divisors(x).iter().fold(0, |acc, y| acc + y * y)))
        .filter(|&(_, z)| {
            let flag = (z as f64).sqrt();
            flag == flag.trunc()
        })
        .collect::<Vec<(u64, u64)>>()
}

fn divisors(integer: u64) -> Vec<u64> {
    let mut divisors = Vec::new();
    let square_root = (integer as f64).sqrt() as u64 + 1;
    for num in 1..square_root {
        if integer % num == 0 && num * num != integer {
            divisors.push(num);
            divisors.push(integer / num);
        }
        if integer % num == 0 && num * num == integer { divisors.push(num); }
    }
    divisors.sort();
    divisors
}
