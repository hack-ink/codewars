fn ez_vec(str: &str, terminating_byte: u8) -> Vec<u8> {
    let mut v8 = str.chars().collect::<String>().into_bytes();
    v8.push(terminating_byte);
    v8
}

fn brain_luck(code: &str, input: Vec<u8>) -> Vec<u8> {
    let code = code.as_bytes();
    let mut input = input.into_iter();
    let mut output = vec![];
    let mut data = [0u8; 5000];
    let mut cp = 0;
    let mut dp = 0;
    while cp < code.len() {
        match code[cp] {
            b'>' => dp += 1,
            b'<' => dp -= 1,
            b'+' => data[dp] = data[dp].wrapping_add(1),
            b'-' => data[dp] = data[dp].wrapping_sub(1),
            b'.' => output.push(data[dp]),
            b',' => if let Some(input) = input.next() { data[dp] = input },
            b'[' if data[dp] == 0 => cp += jump(code[cp..].iter()),
            b']' if data[dp] != 0 => cp -= jump(code[0..cp + 1].iter().rev()),
            _ => {}
        }
        cp += 1;
    }
    output
}

fn jump<'a, I: 'a>(code: I) -> usize
    where I: Iterator<Item=&'a u8>
{
    let mut n = 0;
    for (i, &c) in code.enumerate() {
        if c == b'[' { n += 1; }
        if c == b']' { n -= 1; }
        if n == 0 { return i; }
    }
    unreachable!();
}
