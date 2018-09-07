fn apparently(string: &str) -> String {
    string.split_whitespace()
        .fold((vec![], false), |(mut v, flag), word| {
                v.push(word);

                match word {
                    "apparently" => if flag {
                        v.pop();
                        (v, !flag)
                    } else { (v, flag) }
                    "but" | "and" => {
                        v.push("apparently");
                        (v, true)
                    }
                    _ => (v, false)
                }
            }
        ).0
        .join(" ")
}

fn test_exp(a: &str, exp: &str) {
    assert_eq!(apparently(a), exp.to_string());
}

#[test]
fn test_apparently() {
    test_exp("It was great and I have never been on live television before but sometimes I dont watch this.", "It was great and apparently I have never been on live television before but apparently sometimes I dont watch this.");
    test_exp("and", "and apparently");
    test_exp("apparently", "apparently");
}
