fn choose_best_sum(t: i32, k: i32, ls: &Vec<i32>) -> i32 {
    if (ls.len() as i32) < k { return -1; }
    let ls1;
    let mut d;
    let mut ls2;
    {   // make ls1 immutable and drop useless variables.
        let mut ls = ls.clone();
        ls.sort();
        let pivot = ls.len() - k as usize;
        d = ls[pivot..].to_vec();
        let mut ls1_ = ls[..pivot].to_vec();
        ls1_.reverse();
        ls2 = ls1_.clone();
        ls1_.push(d[0]);
        ls1 = ls1_;
    }
    ls2.push(d[1]);
    let edge = ls1.len() - 1;
    let mut d_p = 1;
    let mut ls1_p = 0;
    let mut ls2_p = 0;
    loop {
        println!("{:?}, {:?}, {:?}", d, ls1, ls2);
        match d.iter().sum::<i32>() {
            sum if sum > t => { println!("{}", sum);d[0] = ls1[ls1_p]}
            sum @ _ => return sum
        }
        if ls1_p == edge {
            ls1_p = 0;
            d[d_p] = ls2[ls2_p];
            if ls2_p == edge {
                ls2_p = 0;
                d_p += 1;
                if d_p as i32 == k { return -1; }
                ls2.pop();
                ls2.push(d[d_p]);
                d[d_p] = ls2[ls2_p];
            }
            ls2_p += 1;
        } else { ls1_p += 1; }
    }
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