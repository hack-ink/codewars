static WORDS: &'static [&'static str] = &[
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
    "twenty",
];

struct Arith {
    value: &'static str,
}

impl Arith {
    fn add(&self, s: &str) -> &str {
        let a = WORDS.iter().position(|&x| x == self.value).unwrap();
        let b = WORDS.iter().position(|&x| x == s).unwrap();
        &WORDS[a + b]
    }
}
