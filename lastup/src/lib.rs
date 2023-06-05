pub fn lastup(input: &str) -> String {
    let mut vec_result = vec![];
    input.to_lowercase().split_whitespace().for_each(|word| {
        let mut word = word.to_string();
        capitalize_end(&mut word);
        vec_result.push(word)
    });
    vec_result.join(" ")
}

fn capitalize_end(input:&mut String) {
    let mut output_vec: Vec<char> = input.chars().collect();
    let mut last_i = output_vec.len()-1;
    while !output_vec[last_i].is_alphabetic() { last_i -= 1 };
    output_vec[last_i] = output_vec[last_i].to_ascii_uppercase();
    *input = output_vec.iter().collect::<String>();
}
