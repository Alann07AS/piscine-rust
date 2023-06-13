pub fn delete_prefix<'a>(prefix: &str, s: &'a str) -> Option<&'a str> {
    if s[..prefix.len()].to_owned() == prefix {
        Some(&s[prefix.len()-1..])
    } else {
        None
    }
}

