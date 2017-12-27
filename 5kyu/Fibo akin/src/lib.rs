fn un(n: i32) -> Vec<i32> {
    let mut un = vec![1, 1];
    for i in 2..n as usize {
        let next = un[i - un[i - 1] as usize] + un[i - un[i - 2] as usize];
        un.push(next);
    }
    un
}

fn length_sup_uk(n: i32, k: i32) -> i32 {
    un(n).into_iter().filter(|&x| x >= k).count() as i32
}

fn comp(n: i32) -> i32 {
    let un = un(n);
    let mut count = 0;
    for i in 2..un.len() {
        if un[i] < un[i - 1] { count += 1; }
    }
    count
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