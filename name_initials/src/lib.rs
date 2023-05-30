pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.iter().map(|x| {
        let split_vec:  Vec<&str> = x.split_whitespace().collect();
        if let (Some(first_char), Some(second_char)) = (split_vec[0].chars().next(), split_vec[1].chars().next()) {
            format!("{}. {}.",first_char, second_char).to_string()
        } else {
            String::from("")
        }
    })
    .collect() //.collect::<Vec<String>>()
}
