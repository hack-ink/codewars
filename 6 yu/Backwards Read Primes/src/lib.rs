fn backwards_prime(start: u64, stop: u64) -> Vec<u64> {
    (start..stop + 1)
        .filter(|&x|
            is_prime(x) && {
                let y = x.to_string().chars().rev().collect::<String>().parse::<u64>().unwrap();
                y != x && is_prime(y)
            }).collect()
}

fn is_prime(num: u64) -> bool {
    //    special number
    if num == 2 || num == 3 { return true; }
    //    cant be prime
    if num % 6 != 1 && num % 6 != 5 { return false; }
    let tmp = (num as f64).sqrt() as u64;
    let mut i: u64 = 5;
    //    may be prime
    while i <= tmp {
        if num % i == 0 || num % (i + 2) == 0 { return false; }
        i += 6;
    }
    true
}
