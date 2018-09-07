fn prime_factors(n: i32) -> i32 {
    let mut number = n;
    let mut factor = 2;
    let mut sum = 0;
    let mut count;
    while number > 1 {
        count = 0;
        while number % factor == 0 {
            number /= factor;
            count += 1;
        }
        if count > 0 { sum += count }
        factor += 1;
    }
    sum
}

fn consec_kprimes(k: i32, arr: Vec<i32>) -> i32 {
    let all = arr
        .into_iter()
        .map(|x| prime_factors(x))
        .collect::<Vec<i32>>()
    ;
    let mut count = 0;
    for i in 1..all.len() { if all[i - 1] == k && all[i - 1] == all[i] { count += 1; } }
    count
}
