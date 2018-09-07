fn sum_of_n(n: i32) -> Vec<i32> {
    let mut v: Vec<i32> = if n > 0 { 
                              (0..n + 1).collect()
                          } else { 
                              (n..1).rev().collect()
                          };
    for i in 1..v.len() {
        v[i] += v[i - 1];
    }
    v
}
