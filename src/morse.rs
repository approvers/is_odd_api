use num_bigint::BigUint;
use once_cell::sync::Lazy;
use std::collections::HashMap;

fn decode_morse(encoded: &str) -> Option<String> {
    static TABLE: Lazy<HashMap<[&str; 2], &str>> = Lazy::new(|| {
        let mut table = HashMap::with_capacity(10);
        table.insert([".----", ".-"], "1");
        table.insert(["..---", "..-"], "2");
        table.insert(["...--", "...-"], "3");
        table.insert(["....-", "....-"], "4");
        table.insert([".....", "....."], "5");
        table.insert(["-....", "-...."], "6");
        table.insert(["--...", "-..."], "7");
        table.insert(["---..", "-.."], "8");
        table.insert(["----.", "-."], "9");
        table.insert(["-----", "-"], "0");
        table
    });

    let mut decoded_string: Vec<&'static str> = vec![];

    for code in encoded.split(' ') {
        let v = TABLE.iter().find(|x| x.0.contains(&code))?;
        decoded_string.push(*v.1);
    }

    Some(decoded_string.join(""))
}

pub fn morse_code_translate(code: &str) -> Option<BigUint> {
    let morse = decode_morse(code.trim())?;
    Some(morse.parse().expect("failed to parse decoded digits"))
}
