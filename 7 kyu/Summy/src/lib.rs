fn summy(strng: &str) -> i32 {
    strng.split_whitespace().map(|x| x.parse::<i32>().unwrap()).sum()
}
