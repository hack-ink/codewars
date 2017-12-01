fn parse(code: &str) -> Vec<i32> {
    let mut v = vec![];
    let mut x = 0;
    for c in code.chars() {
        match c {
            'i' => x += 1,
            'd' => x -= 1,
            's' => x *= x,
            'o' => v.push(x),
            _ => (),
        }
    }
    v
}
