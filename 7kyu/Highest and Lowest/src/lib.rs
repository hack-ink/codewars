fn high_and_low(numbers: &str) -> String {
    let vector_string: Vec<&str> = numbers.split(" ").collect();
    let max: i32 = vector_string.iter().map(|x| x.parse::<i32>().unwrap()).max().unwrap();
    let min: i32 = vector_string.iter().map(|x| x.parse::<i32>().unwrap()).min().unwrap();
    format!("{:?} {:?}", max, min)
}