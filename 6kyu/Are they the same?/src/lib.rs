fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    let mut a1 = a.iter().map(|&x| x * x).collect::<Vec<_>>();
    let mut a2 = b;
    a1.sort();
    a2.sort();
    a1 == a2
}

/*fn comp(a: Vec<i64>, b: Vec<i64>) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut a = a;
    let mut b = b;
    a.sort_by_key(|k| k.abs());
    b.sort();
    for i in 0..a.len() {
        if a[i] * a[i] != b[i] {
            return false;
        }
    }
    true
}*/
