fn sum_pairs(ints: &[i8], s: i8) -> Option<(i8, i8)> {
    let v: Vec<(usize, i8)> = ints.to_vec().into_iter().enumerate().collect();
    let mut result = None;
    result
}

fn main() {
    let l1 = [1, 4, 8, 7, 3, 15];
    println!("{:?}", sum_pairs(&l1, 8));
}
