fn new_avg(arr: &[f64], newavg: f64) -> Option<i32> {
    match (arr.len() as f64 + 1.) * newavg - arr.iter().sum::<f64>() {
        n if n > 0. => Some(n.ceil() as i32),
        _ => None
    }
}