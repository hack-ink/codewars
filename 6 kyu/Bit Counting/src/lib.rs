fn count_bits(n: i64) -> u32 {
    n.count_ones()
}

//fn count_bits(n: i64) -> u32 {
//    let mut n = n;
//    let mut m = 0;
//
//    loop {
//        match n {
//            0 => return 0,
//            1 => return m + 1,
//            _ => {
//                if n & 1 == 1 { m += 1; }
//
//                n = n >> 2 - 1;
//            }
//        }
//    }
//}

#[test]
fn returns_expected() {
    assert_eq!(count_bits(0), 0);
    assert_eq!(count_bits(4), 1);
    assert_eq!(count_bits(7), 3);
    assert_eq!(count_bits(9), 2);
    assert_eq!(count_bits(10), 2);
    assert_eq!(count_bits(1234), 5);
}