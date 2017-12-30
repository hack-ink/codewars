/*use std::collections::HashSet;

fn trotter(n: i32)-> i32{
    if n == 0 { return -1; }
    let mut t = 1;
    let mut d: Vec<HashSet<char>> = vec![];
    loop {
        d.push((n * t).to_string().chars().collect());
        let u: HashSet<char> = d.iter().fold(HashSet::new(), |acc, x| x.union(&acc).cloned().collect());
        if u.len() == 10 { return n * t; } else { d = vec![u]; }
        t += 1;
    }
}*/

fn trotter(n: i32) -> i32 {
    if n == 0 {
        -1
    } else {
        let mut saw = 0_u16;
        let mut res = 0;
        while saw != 0b1111111111 {
            res += n;
            let mut bleatrix = res;
            while bleatrix > 0 {
                saw |= 1 << bleatrix % 10;
                bleatrix /= 10;
            }
        }
        res
    }
}
