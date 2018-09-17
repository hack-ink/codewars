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

    fn build_bits(bits: &mut Vec<(char, u32)>, prev_c: char, count: u32, shortest: &mut (u32, u32)) {
        bits.push((prev_c, count));

        match prev_c {
            '0' => if count < shortest.0 { shortest.0 = count; }
            '1' => if count < shortest.1 { shortest.1 = count; }
            _ => unreachable!()
        }
    }

    pub fn decode_bits(&self, encoded: &str) -> String {
        let mut bits = vec![];
        let mut shortest = (10, 10);
        {
            let mut prev_c = '1';
            let mut count = 0u32;
            for c in encoded.trim_matches('0').chars() {
                if prev_c != c {
                    MorseDecoder::build_bits(&mut bits, prev_c, count, &mut shortest);

                    prev_c = c;
                    count = 1;
                } else { count += 1; }
            }

            MorseDecoder::build_bits(&mut bits, prev_c, count, &mut shortest);
        }

        let mut morse = String::new();
        let (rate_3, rate_7) = if bits.len() < 3 { (shortest.1 * 3, shortest.1 * 7) } else {
            let (ones, zeros) = (shortest.1, shortest.0);
            if ones == zeros / ones * ones || ones * 3 == zeros / 3 * 3 || ones * 7 == zeros / 7 * 7 {
                (ones * 3, ones * 7)
            } else { (zeros * 3, zeros * 7) }
        };

        for (c, count) in bits.into_iter() {
            match c {
                '1' => if rate_3 == count { morse.push('-'); } else { morse.push('.'); }
                '0' => {
                    if rate_3 == count { morse.push(' '); }
                    if rate_7 == count { morse.push('\\'); }
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
                        word + &self.morse_code.get(code).unwrap(),
                    )
            ).collect::<Vec<_>>()
            .join(" ")
    }
}

#[test]
fn examples() {
    let decoder = MorseDecoder::new();
    assert_eq!(decoder.decode_morse(&decoder.decode_bits("1100110011001100000011000000111111001100111111001111110000000000000011001111110011111100111111000000110011001111110000001111110011001100000011")), "HEY JUDE".to_string());
    assert_eq!(decoder.decode_morse(&decoder.decode_bits("111000000000111")), "EE".to_string());
    assert_eq!(decoder.decode_morse(&decoder.decode_bits("10001")), "EE".to_string());
    assert_eq!(decoder.decode_morse(&decoder.decode_bits("01110")), "E".to_string());
    assert_eq!(decoder.decode_morse(&decoder.decode_bits("1110111")), "M".to_string());
    assert_eq!(decoder.decode_morse(&decoder.decode_bits("111000111")), "I".to_string());
    assert_eq!(decoder.decode_morse(&decoder.decode_bits("11111100111111")), "M".to_string());
    assert_eq!(decoder.decode_morse(&decoder.decode_bits("00011100010101010001000000011101110101110001010111000101000111010111010001110101110000000111010101000101110100011101110111000101110111000111010000000101011101000111011101110001110101011100000001011101110111000101011100011101110001011101110100010101000000011101110111000101010111000100010111010000000111000101010100010000000101110101000101110001110111010100011101011101110000000111010100011101110111000111011101000101110101110101110")), "THE QUICK BROWN FOX JUMPS OVER THE LAZY DOG.".to_string());
}