use std::collections::HashMap;

fn dec2_fact_string(nb: u64) -> String {
    unimplemented!()
}

fn fact_string_2dec(s: String) -> u64 {
    let alphabet: HashMap<char, u64> = (b'A'..b'Z' + 1).map(|c| c as char).zip(10..).collect();
    println!("{:?}", alphabet);
    unimplemented!()
}

fn main() {
    //println!("{}", dec2_fact_string(2982));
    println!("{}", fact_string_2dec("4041000".to_string()));
}
