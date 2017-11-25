fn trotter(n: i32) -> i32 {
    assert!(n >= 0);
    if n == 0 {
        -1
    } else {
        let mut saw = 0_u16;
        let mut res = 0;
        while saw != 0b1_111_111_111 {
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