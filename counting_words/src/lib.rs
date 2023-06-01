use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut result_map = HashMap::<String, u32>::new();
    words.replace(&['(', ')', '“', '”', ',', '\"', '.', ';', '!', ':', '―'][..], "")
        .split_whitespace()
        .for_each(|word| {
            *result_map.entry(word.to_string().to_lowercase()).or_insert(0) += 1;
        });
    result_map
}