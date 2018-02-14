fn interpreter(tape: &str, data: &str) -> String {
    let mut instructions = tape.chars().cycle();
    let mut bits: Vec<char> = data.chars().collect();
    
    bits.into_iter().map(|mut c| {
        while instructions.next().unwrap() == '1' {
            if c == '0' { c = '1'; } else { c = '0'; }
        }
        c
    }).collect()
}
