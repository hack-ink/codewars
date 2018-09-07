fn repeat(s: &str, n: usize) -> String {
    std::iter::repeat(s).take(n).collect()
}

fn num_as_roman(num: i32) -> String {
    num
        .to_string()
        .chars()
        .map(|c: char| c
            .to_digit(10)
            .unwrap() as usize
        )
        .rev()
        .enumerate()
        .map(|(digit, num): (usize, usize)|
            {
                let mut r1 = "";
                let mut r5 = "";
                let mut r10 = "";
                let mut result = String::new();
                match digit {
                    0 =>
                        {
                            r1 = "I";
                            r5 = "V";
                            r10 = "X"
                        }
                    1 =>
                        {
                            r1 = "X";
                            r5 = "L";
                            r10 = "C";
                        }
                    2 =>
                        {
                            r1 = "C";
                            r5 = "D";
                            r10 = "M";
                        }
                    3 => { r1 = "M"; }
                    _ => {}
                }
                match num {
                    1 ... 3 => result = repeat(r1, num),
                    4 => result = r1.to_string() + r5,
                    5 ... 8 => result = r5.to_string() + &repeat(r1, num - 5),
                    9 => result = r1.to_string() + r10,
                    _ => {}
                }
                result
            }
        )
        .collect::<Vec<String>>()
        .into_iter()
        .rev()
        .collect::<Vec<String>>()
        .join("")
}

//fn num_as_roman(mut num: i32) -> String {
//    let mut letters = String::new();
//    let symbols = [(1000, "M"), (900, "CM"),
//        (500,  "D"), (400, "CD"),
//        (100,  "C"), (90,  "XC"),
//        (50,   "L"), (40,  "XL"),
//        (10,   "X"), (9,   "IX"),
//        (5,    "V"), (4,   "IV"),
//        (1,    "I")];
//    for &(n, symbol) in symbols.iter() {
//        while num >= n {
//            letters.push_str(symbol);
//            num -= n;
//        }
//    }
//    letters
//}
