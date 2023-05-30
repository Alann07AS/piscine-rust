
pub fn first_subword(mut s: String) -> String {
    let mut i_end_word = 0;
    for (index, ch) in s.chars().enumerate() {
        if (ch == ' ' || ch == '_' || ch.is_ascii_uppercase()) && index != 0 {
            i_end_word = index;
            break;
        }
    };
    if i_end_word != 0 {s.truncate(i_end_word)};
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = first_subword(2, 2);
        assert_eq!(result, 4);
    }
}
