use std::collections::HashMap;

pub fn is_pangram(s: &str) -> bool {
    let mut vec: Vec<char> = vec![];
    s.to_lowercase().chars()
    .for_each(|ch| {
        if ch.is_ascii_alphabetic() {
            if !vec.contains(&ch) {vec.push(ch)}
        }
    });
    vec.len() == 26
}