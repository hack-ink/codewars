/*fn rolldice_sum_prob(sum:i32, dice_amount:i32) -> f64 {
    let dice_amount = dice_amount as usize;
    let mut dices = vec![];
    dices.resize(dice_amount, 1);
    let mut candidates = 0;
    loop {
        for v in 2..8 {
            if dices.iter().sum::<i32>() == sum { candidates += 1; }
            dices[0] = v;
        }
        for i in 0..dice_amount {
            if dices[i] == 7 {
                if i + 1 == dice_amount { return candidates as f64 / 6u64.pow(dice_amount as u32) as f64; }
                dices[i] = 1;
                dices[i + 1] += 1;
                if dices[i + 1] != 7 { break; }
            }
        }
    }
}*/

fn rolldice_sum_prob(sum: i32, dice_amount: i32) -> f64 {
    if sum < dice_amount || sum > 6 * dice_amount {
        0f64
    } else if dice_amount == 0 {
        1f64
    } else {
        (1..7)
            .map(|d| rolldice_sum_prob(sum - d, dice_amount - 1))
            .sum::<f64>() / 6f64
    }
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