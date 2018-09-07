fn partitions(n: isize) -> isize {
    if n == 0 {
        return 0;
    }
    let n = n as usize;
    let mut c1 = Vec::new();
    let mut c2 = Vec::new();
    c1.resize(n + 1, 1);
    c2.resize(n + 1, 0);
    for i in 2..n + 1 {
        for j in 0..n + 1 {
            let mut k = 0;
            while k + j <= n {
                c2[k + j] += c1[j];
                k += i;
            }
        }
        for j in 0..n + 1 {
            c1[j] = c2[j];
            c2[j] = 0;
        }
    }
    c1[n]
}
