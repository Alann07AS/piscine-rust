pub fn to_url(s: &str) -> String {
    s.replace(" ", "%20").to_string()
}
