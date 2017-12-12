fn dec(nb: u64) -> (char, u64) {
    let mut n = 0;
    let mut fac = 1;
    loop {
        n += 1;
        let tmp = fac * n;
        if tmp > nb {
            for i in 3.. {
                if fac * i > nb {
                    return ((i - 1).to_string().chars().next().unwrap(), fac);
                }
            }
        }
        fac = tmp;
    }
}

fn dec2_fact_string(nb: u64) -> String {
    let mut s = String::new();
    let mut nb = nb;
    while nb != 0 {
        let (c, n) = dec(nb);
        nb -= n;
        s.push(c);
    }
    s
}

fn fact_string_2dec(s: String) -> u64 {
    unimplemented!()
}

fn main() {
    println!("{}", dec2_fact_string(2982));
}
