fn my_first_interpreter(code: &str) -> String {
    let mut s = String::new();
    let mut mem: u8 = 0;
    for c in code.chars() {
        match c {
            '+' => mem = mem.wrapping_add(1),
            '.' => s.push(mem as char),
            _ => ()
        }
    }
    s
}