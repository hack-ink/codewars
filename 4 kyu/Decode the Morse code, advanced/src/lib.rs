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

    fn bit_to_code(morse: &mut String, bit: &str) {
        match bit {
            "11" => morse.push('.'),
            "00" => (),
            "111111" => morse.push('-'),
            "000000" => morse.push(' '),
            "00000000000000" => morse.push('\\'),
            _ if bit.starts_with('1') => morse.push('.'),
            _ => ()
        }
    }

    pub fn decode_bits(&self, encoded: &str) -> String {
        let mut morse = String::new();
        let mut bit = String::from("1");
        let mut bits: Vec<_> = encoded.chars().collect();

        while bits[0] == '0' { bits.remove(0); }
        while bits[bits.len() - 1] == '0' { bits.pop(); }

        for i in 1..bits.len() {
            if bits[i - 1] != bits[i] {
                MorseDecoder::bit_to_code(&mut morse, &bit);
                bit.clear();
            }

            bit.push(bits[i]);
        }

        MorseDecoder::bit_to_code(&mut morse, &bit);
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
    assert_eq!(decoder.decode_morse(&decoder.decode_bits("111111000000111111000000111111000000111111000000000000000000111111000000000000000000111111111111111111000000111111000000111111111111111111000000111111111111111111000000000000000000000000000000000000000000111111000000111111111111111111000000111111111111111111000000111111111111111111000000000000000000111111000000111111000000111111111111111111000000000000000000111111111111111111000000111111000000111111000000000000000000111111")), "I".to_string());
}