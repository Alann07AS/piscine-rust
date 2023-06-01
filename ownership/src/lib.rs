
pub fn first_subword(mut s: String) -> String {
    for (index, ch) in s.chars().enumerate() {
        if (ch == ' ' || ch == '_' || ch.is_ascii_uppercase()) && index != 0 {
            s.truncate(index);
            break;
        }
    };
    s
}
