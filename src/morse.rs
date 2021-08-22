pub mod morse_code {
    use num_bigint::BigUint;
    use std::collections::HashMap;
    struct MorseDecoder {
        morse_code: HashMap<Vec<String>, String>,
    }

    impl MorseDecoder {
        fn decode_morse(&self, encoded: &str) -> String {
            let refined: Vec<String> = encoded
                .split(" ")
                .map(|s| s.parse().expect("parse error"))
                .collect();
            let mut decoded_string: Vec<String> = vec![];
            for code in refined {
                match self
                    .morse_code
                    .clone()
                    .into_iter()
                    .find(|x| x.0.contains(&code))
                {
                    Some(v) => decoded_string.push(v.1.to_string()),
                    None => return "parse error".to_string(),
                }
            }
            return decoded_string.join("").trim().to_string();
        }
    }

    pub fn morse_code_translate(code: &str) -> Result<BigUint, num_bigint::ParseBigIntError> {
        let morse_key = vec![
            vec![".----".to_string(), ".-".to_string()],
            vec!["..---".to_string(), "..-".to_string()],
            vec!["...--".to_string(), "...-".to_string()],
            vec!["....-".to_string(), "....-".to_string()],
            vec![".....".to_string(), ".....".to_string()],
            vec!["-....".to_string(), "-....".to_string()],
            vec!["--...".to_string(), "-...".to_string()],
            vec!["---..".to_string(), "-..".to_string()],
            vec!["----.".to_string(), "-.".to_string()],
            vec!["-----".to_string(), "-".to_string()],
        ];

        let letters = vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
            "4".to_string(),
            "5".to_string(),
            "6".to_string(),
            "7".to_string(),
            "8".to_string(),
            "9".to_string(),
            "0".to_string(),
        ];

        let decoder = MorseDecoder {
            morse_code: morse_key.into_iter().zip(letters.into_iter()).collect(),
        };
        return decoder.decode_morse(code.trim()).parse();
    }
}
