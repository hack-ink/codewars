fn f(x: f64) -> f64 {
    x / (1. + (1. + x).sqrt())
}

/*fn f(x: f64) -> f64 {
    let ref mut root = (1. + x).sqrt().to_string().chars().collect::<Vec<char>>()[2..].to_vec();
    let mut result = vec!['0', '.'];
    result.append(root);
    result.iter().collect::<String>().parse::<f64>().unwrap()
}*/
