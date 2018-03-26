use std::collections::HashMap;

// Preloaded:
//
struct MorseDecoder {
    morse_code: HashMap<String, String>
}
//
// MorseDecoder::new() populates the morse_code map, e.g. ".-" -> "A".

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
                .collect()
        }
    }

    fn decode_morse(&self, encoded: &str) -> String {
        let mut words = vec![];

        for word in encoded.split("   ") {
            let mut letters = vec![];

            for letter in word.split_whitespace() {
                letters.push(self.morse_code.get(letter).unwrap().clone());
            }

            if !letters.is_empty() { words.push(letters.concat()); }
        }

        words.join(" ")
    }
}

#[test]
fn test_hey_jude() {
    let decoder = MorseDecoder::new();

    assert_eq!(decoder.decode_morse("       .... . -.--   .--- ..- -.. ."), "HEY JUDE");
}