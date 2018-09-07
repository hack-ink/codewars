fn zoom(n: i32) -> String {
    let n = n as usize;
    let half_n = (n + 1) / 2;
    let mut half_square: Vec<String> = vec![];
    let mut outside = '□';
    let mut inside = '■';

    if half_n % 2 != 0 { std::mem::swap(&mut outside, &mut inside) };

    let side = std::iter::repeat(vec![outside, inside]).take(half_n)
        .collect::<Vec<Vec<char>>>()
        .concat();

    for i in 0..half_n {
        let left: String = side.iter().take(i).collect();
        let mid = std::iter::repeat(outside)
            .take(n - 2 * i)
            .collect::<String>();

        half_square.push([left.clone(), mid, left.chars().rev().collect()].concat());
        std::mem::swap(&mut outside, &mut inside);
    }

    let square = [
        half_square.clone(),
        half_square.into_iter().rev().collect::<Vec<String>>()[1..].to_vec()
    ].concat().join("\n");

    println!("{}", square);

    square
}

#[test]
fn basic_test_1() {
    assert_eq!(zoom(1), "■");
}

#[test]
fn basic_test_2() {
    assert_eq!(zoom(3), "\
□□□
□■□
□□□"
    );
}

#[test]
fn basic_test_3() {
    assert_eq!(zoom(5), "\
■■■■■
■□□□■
■□■□■
■□□□■
■■■■■"
    );
}

#[test]
fn basic_test_4() {
    assert_eq!(zoom(7), "\
□□□□□□□
□■■■■■□
□■□□□■□
□■□■□■□
□■□□□■□
□■■■■■□
□□□□□□□"
    );
}

#[test]
fn basic_test_5() {
    assert_eq!(zoom(9), "\
■■■■■■■■■
■□□□□□□□■
■□■■■■■□■
■□■□□□■□■
■□■□■□■□■
■□■□□□■□■
■□■■■■■□■
■□□□□□□□■
■■■■■■■■■"
    );
}