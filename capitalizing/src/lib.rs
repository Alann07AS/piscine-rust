pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn title_case(input: &str) -> String {
    let mut result: Vec<String> = vec![]; // Utiliser Vec<String> au lieu de Vec<&str>
    for word in input.split_whitespace() {
        result.push(capitalize_first(word)); // Ajouter la chaîne de caractères directement dans le vecteur
    }
    result.join(" ").to_string()
}

pub fn change_case(input: &str) -> String {
    let mut result = String::from("");
    for ch in input.chars() {
        if ch.is_uppercase() {
            result += &ch.to_ascii_uppercase().to_string();
        } else {
            result += &ch.to_ascii_lowercase().to_string();
        }
    }
    result
}