use std::collections::HashMap;

enum Status {
    Code,
    Decode,
}

fn init_fac(x: u64, status: Status) -> Vec<u64> {
    match status {
        Status::Code => (),
        Status::Decode => (),
    }
    unimplemented!()
}

fn dec2_fact_string(nb: u64) -> String {
    let fac = init_fac(nb, Status::Code);
    unimplemented!()
}

fn fact_string_2dec(s: String) -> u64 {
    let fac = init_fac(s.len() as u64, Status::Decode);
    let alphabet: HashMap<char, u64> = (b'A'..b'Z' + 1).map(|c| c as char).zip(10..).collect();
    unimplemented!()
}

fn main() {
    //println!("{}", dec2_fact_string(2982));
    println!("{}", fact_string_2dec("4041000".to_string()));
}
