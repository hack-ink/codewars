fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    let (len, k) = (ls.len(), k as usize);
    if len < k { return -1; }
    let pivot = len - k;
    let mut status: Vec<bool> = vec![];
    status.resize(len, false);
    let mut ls = ls.clone();
    ls.sort();
    let mut d: Vec<i32> = vec![];
    {
        let (min, max) = (ls[..k].iter().sum(), ls[pivot..].iter().sum());
        if min > t { return -1; }
        if max <= t { return max; }
        d.push(min);
    }
    for i in 0..k { status[i] = true; }
    loop {
        for i in 1..len {
            if status[i - 1] && !status[i] {
                status.swap(i - 1, i);
                let mut count = status[0..i - 1].iter().filter(|&&t| t).count();
                for j in 0..i - 1 {
                    if count > 0 {
                        status[j] = true;
                        count -= 1;
                    } else { status[j] = false; }
                }
                let sum = status.iter().enumerate().fold(0, |acc, (i, &t)| { if t { ls[i] + acc } else { acc }});
                if sum <= t { d.push(sum); }
                if status[pivot..].iter().all(|&t| t) { return d.into_iter().max().unwrap(); }
                break;
            }
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
    let ts = &vec![253, 244, 167, 97, 348, 249, 296, 321, 81, 433];
    testing(1437, 5, ts, 1436);
}