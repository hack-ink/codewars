fn remove_nb(m: i32) -> Vec<(i32, i32)> {
    let sum = m as i64 * (m as i64 + 1) / 2;
    let mut root = (sum as f64).sqrt() as i32 + 1;
    let mut result = vec![];
    for a in (m / 2..root).rev() {
        if (sum - a as i64) % (a as i64 + 1) == 0 {
            root = ((sum - a as i64) / (a as i64 + 1)) as i32;
            result.append(&mut vec![(a, root), (root, a)]);
        }
    }
    result.sort();
    result
}
