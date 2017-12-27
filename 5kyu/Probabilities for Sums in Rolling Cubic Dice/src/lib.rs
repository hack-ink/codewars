fn rolldice_sum_prob(sum: i32, dice_amount: i32) -> f64 {
    let option = 6u64.pow(dice_amount as u32);
    let mut dices = vec![];
    dices.resize(dice_amount as usize, 1);
    let mut candidates = 0;
    let mut count = 0;
    for i in 0..dice_amount as usize {
        for v in 1..7 {
            if dices.iter().sum::<i32>() == sum { candidates += 1; }
            dices[i] = v;
            count += 1;
        }
    }
    println!("{}", count);
    candidates as f64 / option as f64
}

fn assert_fuzzy_eq(actual: f64, expected: f64, eps: f64) {
    assert!((actual - expected).abs() < eps, format!("Expected {}, but got {}", expected, actual));
}

#[test]
fn returns_expected() {
    assert_fuzzy_eq(rolldice_sum_prob(11, 2), 0.055555555555, 1e-10);
    assert_fuzzy_eq(rolldice_sum_prob(8, 2), 0.13888888889, 1e-10);
    assert_fuzzy_eq(rolldice_sum_prob(8, 3), 0.0972222222222, 1e-10);
}