pub fn rotate(input: &str, key: i8) -> String {
    let mut result = String::new();

    for c in input.chars() {
        if c.is_alphabetic() {
            let base = if c.is_uppercase() { b'A' } else { b'a' };
            let rotated = ((c as u8 - base + (key.rem_euclid(26) as u8 + 26)) % 26 + base) as char;
            result.push(rotated);
        } else {
            result.push(c);
        }
    }

    result
}
