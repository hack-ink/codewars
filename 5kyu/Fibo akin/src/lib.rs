fn length_sup_uk(n: i32, k: i32) ->i32 {
    unimplemented!()
}
fn comp(n: i32) ->i32 {
    unimplemented!()
}

fn dotest1(n: i32, k: i32, exp: i32) -> () {
    assert_eq!(length_sup_uk(n, k), exp)
}
fn dotest2(n: i32, exp: i32) -> () {
    assert_eq!(comp(n), exp)
}

#[test]
fn test_length_sup_uk() {
    dotest1(50, 25, 2);
    dotest1(3332, 973, 1391);
    dotest1(2941, 862, 1246);
    dotest1(3177, 573, 2047);
}
#[test]
fn test_comp() {
    dotest2(74626, 37128);
    dotest2(71749, 35692);
    dotest2(56890, 28281);
    dotest2(60441, 30054);
}