fn n_linear(m: &[u32], n: usize) -> u32 {
    if n == 0 { return 1; }
    if n < m.len() { return m[n] + 1; }

    let mut groups: Vec<Vec<u32>> = m.iter().map(|&y| vec![y + 1]).collect();
    let mut x = m.iter().min().unwrap() + 1;
    let mut t = 0;

    let mut check = vec![];

    loop {
        t += 1;

        for (i, group) in groups.iter_mut().enumerate() {
            let x = x * m[i] + 1;

            if !group.contains(&x) { group.push(x); }
        }

        println!("{:?}, {}", groups, x);

        let mut i = 0;

        for group in 1..groups.len() {
            if groups[group - 1][0] > groups[group][0] {
                i = group;
            }
        }

        if x == groups[i][0] {
            groups[i].remove(0);

            continue;
        }

        x = groups[i].remove(0);

        check.push(x);

        if n == t {
            println!("{:?}", check);
            return x;
        }
    }
}

#[test]
fn pair_test() {
    assert_eq!(n_linear(&[2, 3], 10), 22);
    assert_eq!(n_linear(&[3, 2], 10), 22);
}

#[test]
fn triplet_test() {
    assert_eq!(n_linear(&[5, 7, 8], 10), 64);
    assert_eq!(n_linear(&[5, 7, 8], 11), 65);
}

#[test]
fn try() {
    println!("{}", n_linear(&[8, 4, 2], 6));
}