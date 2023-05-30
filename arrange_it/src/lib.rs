pub fn arrange_phrase(phrase: &str) -> String {
    let words: Vec<&str> = phrase.split_whitespace().collect();
    let mut new_words: Vec<String> = vec!["".to_string(); words.len()];
    for word in words {
        for ch in word.chars() {
            if ch.is_numeric() {
                let index = (ch as usize) - 48 - 1;
                new_words[index] = word.replace(ch, "");
            }
        }
    }
    new_words.join(" ")
}

