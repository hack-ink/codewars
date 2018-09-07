fn xo(string: &'static str) -> bool {
    string.chars().fold(0, |a, c| {
        match c {
            'x' | 'X' => a + 1,
            'o' | 'O' => a - 1,
            _ => a
        }
    }) == 0
}