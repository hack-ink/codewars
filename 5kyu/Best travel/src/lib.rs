fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    let mut ls = ls.clone();
    ls.sort();
    let avg = ls.iter().sum::<i32>() / ls.len() as i32;
    println!("{:?}, {}", ls, avg);
    1
}

fn main() {
    let ts = &vec![91, 73, 73, 85, 73, 81, 87];
    println!("{}", choose_best_sum(230, 3, ts));
    let ts = &vec![50, 55, 56, 57, 58];
    println!("{}", choose_best_sum(163, 3, ts));
}
