use std::collections::HashMap;

enum Status {
    Code,
    Decode,
}

fn init_fac(x: u64, status: Status) -> Vec<u64> {
    let mut n = 0;
    let mut s = 1;
    let mut v = vec![];
    match status {
        Status::Code => while s < x {
            v.push(s);
            n += 1;
            s *= n;
        }
        Status::Decode => while n < x {
            v.push(s);
            n += 1;
            s *= n;
        }
    }
    v
}

fn dec2_fact_string(nb: u64) -> String {
    let alphabet: HashMap<u64, u8> = (10..).zip(b'A'..b'Z' + 1).collect();
    let mut fac = init_fac(nb, Status::Code);
    let mut s = String::new();
    let mut nb = nb;
    while let Some(fac) = fac.pop() {
        match nb / fac {
            x@10...35 => s.push(*alphabet.get(&x).unwrap() as char),
            x => s.push(x.to_string().chars().next().unwrap())
        }
        nb %= fac;
    }
    s
}

fn fact_string_2dec(s: String) -> u64 {
    let alphabet: HashMap<u8, u64> = (b'A'..b'Z' + 1).zip(10..).collect();
    let mut fac = init_fac(s.len() as u64, Status::Decode);
    s.chars().fold(0, |acc, c| match c {
        'A'...'Z' => acc + alphabet.get(&(c as u8)).unwrap() * fac.pop().unwrap(),
        _ => acc + c.to_digit(10).unwrap() as u64 * fac.pop().unwrap()
    })
}
