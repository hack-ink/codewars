fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    let len = ls.len();
    if (len as i32) < k { return -1; }
    let mut d: Vec<i32> = vec![];
    let mut v: Vec<usize> = vec![];
    for i in 0..k { v.push(i); }
    loop {
        println!("{:?}", v);
        if v[0] == len as i32 {
            for i in 1..len {
                if v[i - 1] == len as i32 {
                    v[i - 1] = 1;
                    v[i] += 1;
                    if i + 1 == len {
                        if let Some(max) = d.into_iter().filter(|&d| d < t).max() {
                            return max;
                        } else { return -1; }
                    }
                    if v[i + 1] != len as i32 - 1 { break; }
                }
            }
        } else { v[0] += 1; }
    }
    unimplemented!()
}

fn testing(t: i32, k: i32, ls: &Vec<i32>, exp: i32) -> () {
    assert_eq!(choose_best_sum(t, k, ls), exp)
}

#[test]
fn basics_choose_best_sum() {
//    let ts = &vec![50, 55, 56, 57, 58];
//    testing(163, 3, ts, 163);
//    let ts = &vec![50];
//    testing(163, 3, ts, -1);
    let ts = &vec![91, 74, 73, 85, 73, 81, 87];
    testing(230, 3, ts, 228);
//    testing(331, 2, ts, 178);
}