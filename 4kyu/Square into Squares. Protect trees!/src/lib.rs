fn aux(nb: i64, mut rac: i64) -> Option<Vec<i64>> {
    if nb == 0 { return Some(vec![]); }
    let mut i = rac;
    while i >= ((nb / 2) as f64).sqrt() as i64 + 1 {
        let diff = nb - i * i;
        rac = (diff as f64).sqrt() as i64;
        let ll = aux(diff, rac);
        if let Some(mut x) = ll {
            x.push(i);
            return Some(x);
        }
        i -= 1;
    }
    None
}

fn decompose(n: i64) -> Option<Vec<i64>> { aux(n * n, ((n * n - 1) as f64).sqrt() as i64) }

//fn decompose(n: i64) -> Option<Vec<i64>> {
//    partition(n * n, n)
//}
//
//fn partition(sqrt: i64, n: i64) -> Option<Vec<i64>> {
//    if sqrt < 0 { return None; }
//    if sqrt == 0 { return Some(Vec::new()); }
//
//    for i in (0..n).rev() {
//        let part = partition(sqrt - i * i, i);
//
//        if part != None { return Some([part.unwrap(), vec![i]].concat()); }
//    }
//
//    None
//}
