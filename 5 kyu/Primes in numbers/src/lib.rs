fn prime_factors(n: i64) -> String {
    let mut number = n;
    let mut prime_numbers = vec![];
    let mut factor = 2;
    let mut cnt;
    while number > 1 {
        cnt = 0;
        while number % factor == 0 {
            number /= factor;
            cnt += 1;
        }
        if cnt > 0 { if cnt > 1 { prime_numbers.push(format!("({}**{})", factor, cnt)); } else { prime_numbers.push(format!("({})", factor)); } }
        factor += 1;
    }
    prime_numbers.join("")
}
