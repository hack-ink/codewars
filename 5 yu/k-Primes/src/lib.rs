fn prime_factors(n: i32) -> Vec<(i32, i32)> {
    let mut number = n;
    let mut prime_numbers = vec![];
    let mut factor = 2;
    let mut count;
    while number > 1 {
        count = 0;
        while number % factor == 0 {
            number /= factor;
            count += 1;
        }
        if count > 0 { prime_numbers.push((factor, count)); }
        factor += 1;
    }
    prime_numbers
}

fn count_kprimes(k: i32, start: i32, end: i32) -> Vec<i32> {
    (start..end + 1)
        .filter(|&count| prime_factors(count)
            .iter()
            .fold(0, |acc, &(_, count)| acc + count) == k)
        .collect()
}

fn puzzle(s: i32) -> i32 {
    let one = count_kprimes(1, 1, s);
    let three = count_kprimes(3, 1, s);
    let seven = count_kprimes(7, 1, s);
    let mut result = 0;
    for one in one.iter() {
        for three in three.iter() {
            for seven in seven.iter() {
                if one + three + seven == s { result += 1; }
            }
        }
    }
    result
}
