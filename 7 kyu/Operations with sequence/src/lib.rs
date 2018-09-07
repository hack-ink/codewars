fn calc(array: Vec<i32>) -> i32 {
    array.iter().enumerate().map(|(i, &x)| {
        let mut val = x;
        if x > 0 { val *= val };
        if (i + 1) % 3 == 0 { val *= 3 };
        if (i + 1) % 5 == 0 { val *= -1 };
        val
    }).sum()
}