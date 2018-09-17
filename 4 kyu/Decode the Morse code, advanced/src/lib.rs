use std::collections::HashMap;

struct MorseDecoder {
    pub morse_code: HashMap<String, String>,
}

impl MorseDecoder {
    pub fn new() -> MorseDecoder {
        MorseDecoder {
            morse_code: [
                (".-", "A"), ("-...", "B"), ("-.-.", "C"), ("-..", "D"), (".", "E"), ("..-.", "F"),
                ("--.", "G"), ("....", "H"), ("..", "I"), (".---", "J"), ("-.-", "K"), (".-..", "L"),
                ("--", "M"), ("-.", "N"), ("---", "O"), (".--.", "P"), ("--.-", "Q"), (".-.", "R"),
                ("...", "S"), ("-", "T"), ("..-", "U"), ("...-", "V"), (".--", "W"), ("-..-", "X"),
                ("-.--", "Y"), ("--..", "Z"),
                ("-----", "0"), (".----", "1"), ("..---", "2"), ("...--", "3"), ("....-", "4"),
                (".....", "5"), ("-....", "6"), ("--...", "7"), ("---..", "8"), ("----.", "9"),
                (".-.-.-", "."), ("--..--", ","), ("..--..", "?"), (".----.", "\'"), ("-.-.--", "!"),
                ("-..-.", "/"), ("-.--.", "("), ("-.--.-", ")"), (".-...", "&"), ("---...", ","),
                ("-.-.-.", ";"), ("-...-", "="), (".-.-.", "+"), ("-....-", "-"), ("..--.-", "_"),
                (".-..-.", "\""), ("...-..-", "$"), (".--.-.", "@"), ("...---...", "SOS"),
            ].iter()
                .map(|&(from, to)| (from.to_string(), to.to_string()))
                .collect(),
        }
    }

    pub fn decode_bits(&self, encoded: &str) -> String {
        let mut bits = vec![];
        let mut rate = 1;

        {
            let mut prev_c = '0';
            let mut count = 0;
            for c in encoded.trim_matches('0').chars() {
                if prev_c != c {
                    if prev_c == '1' && count < rate { rate = count; }
                    bits.push((prev_c, count));

                    prev_c = c;
                    count = 1;
                } else { count += 1; }
            }

            if prev_c == '1' && count < rate { rate = count; }
            bits.push((prev_c, count));
        }

        let (three_time_unit, seven_time_unit) = (rate * 3, rate * 7);
        let mut morse = String::new();
        for (c, count) in bits.into_iter() {
            match c {
                '1' => if three_time_unit == count { morse.push('-'); } else { morse.push('.'); }
                '0' => {
                    if three_time_unit == count { morse.push(' '); }
                    if seven_time_unit == count { morse.push('\\'); }
                }
                _ => unreachable!()
            }
        }

        morse
    }

    pub fn decode_morse(&self, encoded: &str) -> String {
        encoded.split("\\")
            .map(|word|
                word.split_whitespace()
                    .fold(String::new(), |word, code|
                        word + &self.morse_code.get(code).unwrap()
                    )
            ).collect::<Vec<_>>()
            .join(" ")
    }
}

#[test]
fn examples() {
    let decoder = MorseDecoder::new();
//    assert_eq!(decoder.decode_morse(&decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
//    assert_eq!(decoder.decode_morse(&decoder.decode_bits("111000000000111")), "EE".to_string());
//    assert_eq!(decoder.decode_morse(&decoder.decode_bits("10001")), "EE".to_string());
//    assert_eq!(decoder.decode_morse(&decoder.decode_bits("01110")), "E".to_string());
    assert_eq!(decoder.decode_morse(&decoder.decode_bits("1110111")), "M".to_string());
}