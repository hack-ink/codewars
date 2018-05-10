fn n_linear(m: &[u32], n: usize) -> u32 {
    if n == 0 { return 1; }
    if n < m.len() { return m[n - 1] + 1; }

    let mut groups: Vec<Vec<u32>> = m.iter().map(|&y| vec![y + 1]).collect();
    let mut x = m.iter().min().unwrap() + 1;
    let mut t = 1;

    loop {
        let len = x.to_string().len();

        for (i, group) in groups.iter_mut().enumerate() {
            if len + m[i].to_string().len() > 6 { continue; }

            group.push(x * m[i] + 1);
        }

        let i = groups.iter()
            .map(|v| v[0])
            .enumerate()
            .min_by_key(|&(_, x)| x)
            .unwrap()
            .0;

        if x == groups[i][0] {
            groups[i].remove(0);

            continue;
        }

        x = groups[i].remove(0);

        t += 1;

        if n == t { return x; }
    }
}

fn think(m: &[u32], n: usize) -> u32 {
    let mut m = m.to_vec();

    m.sort();

    let mut ratio: Vec<u32> = m[..m.len() - 1].iter()
        .zip(m[1..].iter())
        .map(|(x, y)| y / x)
        .collect();

    let mut v = 1;  // current value
    let mut t = 1;  // loop times
    let mut p = 1;  // period
    let mut i = 0;  // index of m

    loop {
        if p <= m[i + 1] {
            let next = m[i] * v + 1;

            p += 1;
            i += 1;

            if x == next { continue; }

            x = next;
            t += 1;
        }

        if t == n { return x; }
    }
}

#[test]
fn pair_test() {
    assert_eq!(n_linear(&[2, 3], 10), 22);
    assert_eq!(n_linear(&[3, 2], 10), 22);
    assert_eq!(n_linear(&[3, 2], 234), 1339);
}

#[test]
fn triplet_test() {
    assert_eq!(n_linear(&[5, 7, 8], 10), 64);
    assert_eq!(n_linear(&[5, 7, 8], 11), 65);
}

#[test]
fn try() {
    assert_eq!(think(&[5, 7, 8], 11), 65);
}
