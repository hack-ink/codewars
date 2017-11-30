fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn sum_fracts(l: Vec<(i64, i64)>) -> Option<(i64, i64)> {
    l.into_iter().fold(None, |acc, (numerator, denominator)| {
        if let Some((numerator_, denominator_)) = acc {
            let numerator = numerator * denominator_ + numerator_ * denominator;
            let denominator = denominator * denominator_;
            let gcd = gcd(numerator, denominator);
            Some((numerator / gcd, denominator / gcd))
        } else {
            Some((numerator, denominator))
        }
    })
}
