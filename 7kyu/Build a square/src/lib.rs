// nightly only
//#![feature(repeat_generic_slice)]
//fn generate_shape(n: i32) -> String {
//    ["+".repeat(n as usize).as_str()].repeat(n as usize).join("\n")
//}

fn generate_shape(n: i32) -> String {
    use std::iter;

    let mut square: Vec<String> = vec![];

    for _ in 0..n {
        square.push(iter::repeat('+').take(n as usize).collect());
    }

    square.join("\n")
}

#[test]
fn sample_test() {
    assert_eq!(generate_shape(3), "+++\n+++\n+++");
}