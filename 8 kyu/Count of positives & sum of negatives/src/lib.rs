fn count_positives_sum_negatives(input: Vec<i32>) -> Vec<i32> {
    if input.is_empty() { return vec![]; }
    input.iter().fold(vec![0, 0], |mut acc, &x| {
        if x > 0 { acc[0] += 1; } else { acc[1] += x; }
        acc
    })
}