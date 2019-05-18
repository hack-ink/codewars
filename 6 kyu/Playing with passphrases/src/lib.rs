fn play_pass(s: &str, n: u32) -> String {
    let mut passphrases = String::new();

    let (re_downcase, re_upcase) = if s.len() % 2 == 0 { (1, 0) } else { (0, 1) };
    for (i, c) in s.chars().rev().enumerate() {
        match c as u32 {
            c @ 48...57 => passphrases.push((105 - c) as u8 as _),
            mut c @ 65...90 => {
                c += n;
                if c > 90 { c -= 26; }
                if i % 2 == re_upcase { c += 32; }

                passphrases.push(c as u8 as _);
            }
            mut c @ 97...122 => {
                c += n;
                if c + n > 122 { c -= 126; }
                if i % 2 == re_downcase { c -= 32; }

                passphrases.push(c as u8 as _);
            }
            _ => passphrases.push(c)
        }
    }

    passphrases
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(s: &str, n: u32, exp: &str) -> () {
        println!(" s: {:?};", s);
        println!("n: {:?};", n);
        let ans = play_pass(s, n);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest("I LOVE YOU!!!", 1, "!!!vPz fWpM J");
        dotest("I LOVE YOU!!!", 0, "!!!uOy eVoL I");
        dotest("AAABBCCY", 1, "zDdCcBbB");
        dotest("MY GRANMA CAME FROM NY ON THE 23RD OF APRIL 2015", 2, "4897 NkTrC Hq fT67 GjV Pq aP OqTh gOcE CoPcTi aO");
    }
}
