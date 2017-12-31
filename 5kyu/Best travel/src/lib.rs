fn swap_item(a: &mut Vec<i32>, a_ptr: usize, b: &mut Vec<i32>, b_ptr: usize) {
    let tmp = a[a_ptr];
    a[a_ptr] = b[b_ptr];
    b[b_ptr] = tmp;
}

fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    let k = k as usize;
    let mut ls = ls.clone();
    ls.sort();
    let pivot = ls.len() - k as usize;
    let mut d = ls[pivot..].to_vec();
    let mut ls = ls[..pivot].to_vec();
    ls.reverse();
    ls.push(d[0]);
    let edge = ls.len() - 1;
    let mut dp1 = 0;
    let mut lsp1 = 0;
    let mut dp2 = 1;
    let mut lsp2 = 0;
    println!("{:?}, {:?}", d, ls);
    loop {
        if lsp1 == edge {
            lsp1 = 0;
        } else { lsp1 += 1; }
        match d.iter().sum::<i32>() {
            sum if sum > t => d[dp1] = ls[lsp1],
            sum @ _ => return sum
        }
    }
}

fn testing(t: i32, k: i32, ls: &Vec<i32>, exp: i32) -> () {
    assert_eq!(choose_best_sum(t, k, ls), exp)
}

#[test]
fn basics_choose_best_sum() {
    let ts = &vec![50, 55, 56, 57, 58];
    testing(163, 3, ts, 163);
    let ts = &vec![50];
    testing(163, 3, ts, -1);
    let ts = &vec![91, 74, 73, 85, 73, 81, 87];
    testing(230, 3, ts, 228);
    testing(331, 2, ts, 178);
}