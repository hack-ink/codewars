fn high_and_low(numbers: &str) -> String {
    let vector_string: Vec<&str> = numbers.split_whitespace().collect();
    let max: i32 = vector_string.iter().filter_map(|x| x.parse::<i32>()).max().unwrap();
    let min: i32 = vector_string.iter().filter_map(|x| x.parse::<i32>()).min().unwrap();
    format!("{:?} {:?}", max, min)
}