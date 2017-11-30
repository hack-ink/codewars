fn f(x: f64) -> f64 {
    x / (1. + (1. + x).sqrt())
}

/*fn f(x: f64) -> f64 {
    let mut root = (1. + x).sqrt().to_string().chars().collect::<Vec<char>>();
    root[0] = '0';
    root.iter().collect::<String>().parse::<f64>().unwrap()
}*/
