
pub fn first_subword(mut s: String) -> String {
    for (index, ch) in s.chars().enumerate() {
        if (ch == ' ' || ch == '_' || ch.is_ascii_uppercase()) && index != 0 {
            s.truncate(index);
            break;
        }
    };
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
