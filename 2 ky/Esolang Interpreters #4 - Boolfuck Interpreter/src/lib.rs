use std::collections::HashMap;

fn boolfuck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let code = code.as_bytes();
    let mut input = input.into_iter().flat_map(|b| (0u8..8).map(move |i| (b >> i) & 1));
    let mut output = Vec::new();
    let mut tape = HashMap::new();
    let mut stack = Vec::new();
    let mut cp = 0;
    let mut tp = 0;
    while cp < code.len() {
        match code[cp] {
            b'+' =>
                {
                    let t = tape.entry(tp).or_insert(0);
                    *t = if *t == 0 { 1 } else { 0 }
                }
            b',' => { input.next().map(|x| tape.insert(tp, x)); }
            b';' => output.push(*tape.get(&tp).unwrap_or(&0)),
            b'<' => tp -= 1,
            b'>' => tp += 1,
            b'[' => if *tape.get(&tp).unwrap_or(&0) == 0 { cp = matching_bracket(code, cp).unwrap(); } else { stack.push(cp); },
            b']' => cp = stack.pop().unwrap().wrapping_sub(1),
            _ => (),
        }
        cp = cp.wrapping_add(1);
    }
    output.chunks(8).map(|b| b.iter().rev().fold(0, |acc, x| (acc << 1) | x)).collect()
}

fn matching_bracket(code: &[u8], open: usize) -> Option<usize> {
    let mut stack = 0;
    for (i, &c) in code[open..].iter().enumerate() {
        match c {
            b'[' => stack += 1,
            b']' => {
                stack -= 1;
                if stack == 0 { return Some(open + i); }
            }
            _ => (),
        }
    }
    None
}
