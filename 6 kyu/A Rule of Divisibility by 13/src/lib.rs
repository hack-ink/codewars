fn thirt(n: i64) -> i64 {
    let remainder = vec![1, 10, 9, 12, 3, 4];
    let mut result = n;
    loop {
        let mut remainder = remainder.iter().cycle();
        let sequence: Vec<i64> = result
            .to_string()
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as i64)
            .collect();
        let result_ = sequence.into_iter().fold(0, |acc, x| {
            acc + x * remainder.next().unwrap()
        });
        if result == result_ {
            return result;
        }
        result = result_;
    }
}
