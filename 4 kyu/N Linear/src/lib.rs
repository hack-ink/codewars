fn n_linear(m: &[u32], n: usize) -> u32 {
    let mut xs = vec![1];
    let mut x_i: Vec<usize> = vec![0; m.len()];
    let mut u: Vec<(u32, usize)> = m.iter()
        .enumerate()
        .map(|(i, &x)| (x + 1, i))
        .collect();

    let mut t = 0;
    let mut x = 1;

    loop {
        if t == n { return x; }

        match u.iter().min().unwrap() {
            &(next, _) if next != x => {
                xs.push(next);

                x = next;
                t += 1;
            }
            &(_, i) => {
                x_i[i] += 1;
                u[i].0 = m[i] * xs[x_i[i]] + 1;
            }
        }
    }
}
