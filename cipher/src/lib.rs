#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    validation: bool,
    expected: String
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError{validation, expected}
    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {
    let cipher_good = cyphered(original);
    if original.is_empty() || ciphered.is_empty() {
        None
    } else if ciphered == cipher_good {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, cipher_good)))
    }
}

const A_MAJ: u8 = 'A' as u8;
const A_MIN: u8 = 'a' as u8;

fn cyphered(original: &str) -> String {
    let mut result = String::new();
    original.chars()
    .for_each(|ch| {
        if ch.is_alphabetic() {
            if ch.is_ascii_uppercase() {
                result.push(((A_MAJ + 25) - (ch as u8 - A_MAJ)) as char);
            } else {
                result.push(((A_MIN + 25) - (ch as u8 - A_MIN)) as char);
            }
        } else {
            result.push(ch);
        }
    });
    result
}