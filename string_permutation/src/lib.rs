pub fn is_permutation(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    s1.chars().map(|v| v as u32).sum::<u32>()
    == s2.chars().map(|v| v as u32).sum::<u32>()
}